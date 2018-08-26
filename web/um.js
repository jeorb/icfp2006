var UM = function(){
    
    var um_console;
    var data;
    var current_line;
    var finger = 0;
    var arrays = [data];
    var pointers = [];
    var platter, operator, a, b, c;
    var registers = [0, 0, 0, 0, 0, 0 ,0, 0];
    var cycle = 0;
    var showNext = false;
    var running = false;
    var start_time;
    var total_time;
    var key_buffer = [];
    
    function BufferChar(ch){
        if(ch==0){
            ch = 10;
        }
        key_buffer.push(ch);
        console.log(key_buffer);
        Print(String.fromCharCode(ch), 'user-input');
    }
    
    function BufferString(str){
        var i;
        console.log("Buffering "+str);
        for(i=0; i<str.length; i++){
            BufferChar(str.charCodeAt(i));
        }
    }

    function Init(id){
        um_console = document.getElementById(id);
        um_console.textContent = '';
        current_line = document.createElement('p');
        um_console.appendChild(current_line);
        Print('Welcome to the Virtual Universal Machine\n');
        
        um_console.addEventListener('keypress', function(e){
            console.log(e);
            e.preventDefault();
            e.stopPropagation();
            BufferChar(e.charCode);
        });
        um_console.addEventListener('paste', function(e){
            e.preventDefault();
            e.stopPropagation();
            console.log(e);
            BufferString((e.clipboardData || window.clipboardData).getData('text'));
        });
    }
    
    function Stop(){
        running = false;
        console.log("Stopping at next newline.");
    }
    
    function Resume(){
        if(!running){
            console.log("Resuming.");
            running = true;
            Run();
        } else {
            console.log("Already running.");
        }
    }
    
    function Print(str, class_name){
        current_line.textContent += str;
        if(str.includes('\n')){
            console.log(current_line.textContent);
            current_line = document.createElement('p');
            um_console.appendChild(current_line);
        }
        um_console.scrollTop = um_console.scrollHeight;
    }
    
    function Load(url){
        if(running){
            Stop();
            window.setTimeout(function(){ Load(url); }, 5000);
            return;
        }
        Print('Loading '+url+' ...\n');
        
        var xhr = new XMLHttpRequest();
        xhr.open('GET', url, true);
        xhr.responseType = 'arraybuffer';
        
        xhr.onload = function(e) {
            var buffer = xhr.response;
            var scroll = new Uint8Array(buffer);
            
            var len = scroll.length;
            data = new Uint32Array(len/4);

            var i = 0;
            for(i=0; i < len; i+=4){
                var platter
                    = ((scroll[i]) << 24)
                    + ((scroll[i+1]) << 16)
                    + ((scroll[i+2]) << 8)
                    + (scroll[i+3])
                ;
                data[i/4] = platter;
            }

            Print('Loaded ' + data.length + ' sandstone platters.\n');
            
            finger = 0;
            arrays = [data];
            platter, operator, a, b, c;
            registers = [0, 0, 0, 0, 0, 0 ,0, 0];
            cycle = 0;
            showNext = false;
            running = true;
            start_time = performance.now();
            Run();
        }
        
        xhr.send(new Uint32Array());
    }
    
    function Run(){
        var next_pointer = null;
        main_loop: while(finger < arrays[0].length){
            platter = arrays[0][finger];
            operator = platter >>> 28;
            a = (platter >>> 6) & 0b111;
            b = (platter >>> 3) & 0b111;
            c = platter & 0b111;
            
            switch(operator) {
                case 0:
                    //console.log("Conditional Move");
                    if(registers[c] != 0){
                        registers[a] = registers[b];
                    }
                    break;
                case 1:
                    //console.log("Array Index");
                    registers[a] = arrays[registers[b]][registers[c]];
                    break;
                case 2:
                    //console.log("Array Amendment");
                    arrays[registers[a]][registers[b]] = registers[c];
                    break;
                case 3:
                    //console.log("Addition");
                    registers[a] = (registers[b] + registers[c]) >>> 0;
                    break;
                case 4:
                    //console.log("Multiplication");
                    registers[a] = (registers[b] * registers[c]) >>> 0;
                    break;
                case 5:
                    //console.log("Division");
                    registers[a] = (registers[b] / registers[c]) >>> 0; 
                    break;
                case 6:
                    //console.log("Not-And");
                    registers[a] = (~(registers[b] & registers[c])) >>> 0;
                    break;
                case 7:
                    Print("Halt\n");
                    break main_loop;
                    break;
                case 8:
                    //console.log("Allocation");
                    if(pointers.length>0){
                        next_pointer = pointers.pop();
                    } else {
                        next_pointer = arrays.push(null) - 1;
                    }
                    arrays[next_pointer] = new Uint32Array(registers[c]);
                    registers[b] = next_pointer;
                    break;
                case 9:
                    //console.log("Abandonment");
                    arrays[registers[c]] = null;
                    pointers.push(registers[c]);
                    break;
                case 10:
                    //console.log("Output");
                    Print(String.fromCharCode(registers[c]));
                    if(String.fromCharCode(registers[c])=='\n'){
                        finger++;
                        if(running){
                            window.setTimeout(function(){Run();}, 20); // Let the browser catch its breath
                        } else {
                            console.log("Stopped.");
                        }
                        return;
                    }
                    break;
                case 11:
                    //console.log("Input");
                    console.log('input ', key_buffer);
                    if(key_buffer.length > 0){
                        registers[c] = key_buffer.shift() >>> 0;
                        console.log("Sending char to UM: "+String.fromCharCode(registers[c]));
                    } else {
                        // Wait for input
                        window.setTimeout(function(){Run();}, 1000);
                        return;
                    }
                    break;
                case 12:
                    //console.log("Load Program");
                    if(registers[b] > 0){
                        arrays[0] = arrays[registers[b]].slice();
                    }
                    finger = registers[c];
                    continue;
                    break;
                case 13:
                    //console.log("Orthography");
                    a = (platter >>> 25) & 0b111;
                    registers[a] = platter & 0b1111111111111111111111111;
                    break;
                default:
                    Print("Whoops!!\n");
                    return;
                    break;
                
            }
            
            finger++;
        }
        total_time = performance.now() - start_time;

        Print("Done processing in "+(total_time/1000)+" seconds.\n");
    }
    
    return {
        Init: Init,
        Load: Load,
        Stop: Stop,
        Resume: Resume
    }
}();

UM.Init('um-console');

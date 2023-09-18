let x = 0;

while (x < 25000) {
    function test(t) {
        x = x + t;
    }
    
    console.log(x);
    test(1);
}

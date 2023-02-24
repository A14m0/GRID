// This file implements a prime sieve algorithm. RIP CPU :)

// this implements the sieve of Eratosthenese
num[] sieve(num maximum_num) {
    // this is a very Rust way of initializing an array (tho technically this
    // would not be allowed because arrays are fixed size... oh well :)) 
    bool[] primes = [true; maximum_num];    
    num[] returns = [];
    num p = 2;

    // the main bulk of the algorithm
    while(p*p <= maximum_num) {
        // if the value in primes is still True, is prime
        if(primes[p]) {
            // add it to our return array
            returns.push(p)

            // update all multiples of the prime
            // note: how do we want to handle "step-by"?
            //       do we event want to?
            for(i in p*p..num+1) {
                if(i % p == 0){
                    primes[i] = false
                }
            }
        }

        // move on
        p += 1;
    }
    
    // now we return the primes we found
    return returns;
}


num main() {

    // run for all numbers below 50
    num max_number = 50;
    print("Primes below " + max_number.to_string());
    print(sieve(max_number).to_string());

    return 0;
}
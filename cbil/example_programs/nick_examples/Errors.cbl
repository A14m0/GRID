// This file demonstrates how errors might be handled/interacted with

num this_dies() {
    // here, for explanatory purposes, is a function that returns an errored object
    return dead_number;
}


// this function will fail too, because it encountered an error
num this_also_dies() {
    // .. blah blah blah ..

    num heheh_die = this_dies();
    heheh_die += 1; // <--- triggers data access of errored object. dies here, propigates upward thru call stack

    // .. blah blah blah ..
}

// this function shows how checking the error state of the object DOES NOT die
num no_die() {
    // .. blah blah blah ..

    num heheh_die = this_dies();
    
    // this check does not constitute an access of data, so is ignored by the runtime
    if(heheh_die.is_erroed()) {
        print("He's dead, Jim...");
        return 0;
    }

    heheh_die += 1; // <--- no more dies here :D

    // .. blah blah blah ..
}

// this function shows how we can just disregard the error use check entirely 
num no_die() {
    // .. blah blah blah ..
    num for_later = 0;

    // tells runtime to disregard error violations in this context
    nocheck{
        num heheh_die = this_dies();
        heheh_die += 1; // <--- no more dies here, tho should... :/

        // note tho that the error state is still existant on the object
        // which means...
        for_later = heheh_die;
    }

    // ... that its copied use will still cause issues later on if copied...
    for_later += 1; // <-- dies
    
    // ... assuming thats the model of error checking we go with. The other 
    // option is to make all `nocheck` variables unable to escape `nocheck` 
    // bounds and mutate other variables.


    // .. blah blah blah ..
}


use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // easier access
    other_function();

    println!("Entering block");
    {
        // this `function()` will shadow the other one.
        use crate::deeply::nested::function;
        function();

        println!("Leaving block");
    }

    function();
}

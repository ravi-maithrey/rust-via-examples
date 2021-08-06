fn main() {
    let ctx_lines = 1;
    let needle = "oo"; // what we are searching for
    let quote = "\
    Every face, every shop,
    bedroom window, public-house, and 
    dark square is a picture 
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek through 
    millions of pages?";

    // declaring a vec to store the lines which contain our needle
    let mut tags: Vec<usize> = vec![]; 
    // declaring a vec to store the context lines of the lines which contain needle
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in quote.lines().enumerate(){
        if line.contains(needle){
            tags.push(i); // storing lines which contain needle in this vec

            let v = Vec::with_capacity(2*ctx_lines+1);
            ctx.push(v); // setting the context for each line
        }
    }

    if tags.is_empty(){
        return; // if no match lines are found, early return
    }

    for (i, line) in quote.lines().enumerate(){
        for (j, tag) in tags.iter().enumerate(){
            // getting the lower and upper bound for the context window
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag+ctx_lines;

            // pushing the lines within the context window to context vec
            if(i >= lower_bound) && (i <= upper_bound){
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter(){
        // we use 'ref' near line to tell the compiler we are only borrowing
        // because local_ctx itself is a temp variable, only borrowing is allowed
        for &(i, ref line) in local_ctx.iter(){ 
            let line_num = i+1;
            println!("{}: {}", line_num, line);
        }
    }
}

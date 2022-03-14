#[derive(Debug, Clone)]
enum ImperativeFrame<T> {
    Open(T, T),
    Spare(T),
    Strike(T, T),
}

impl<T> ImperativeFrame<T> 
    where T: 
        Copy +
        From<u8> +
        PartialEq + 
        PartialOrd + 
        std::ops::Add<Output = T > {

    // Imperative implementation
    fn to_frame (game: &[T]) -> Option<Vec<ImperativeFrame<T>>> {

        use ImperativeFrame::{Strike, Spare, Open};
        
        // Create a vector
        let mut vector: Vec<ImperativeFrame<T>> = Vec::new();
        // Create a mutable slice to iterate input data
        let mut next_game = game;

        // Compute the remaining items
        let mut remaining= next_game.len();
        
        while remaining > 0 {

            // Get a frame
            let frame = ImperativeFrame::chunk_to_frame(&next_game)?;
            
            next_game = match frame {

                // If is a Strike or Spare and there are just 3 items remaining then
                // this is the last chunk, and we iterate in a different way to avoid
                // getting a last Open with the Spare or Strike Bonus
                Strike(..) if remaining == 3 => &next_game[remaining..],
                Spare(..) if remaining == 3 => &next_game[remaining..],

                // Normal way to iterate
                Strike(..) => &next_game[1..],
                Spare(..) | Open(..) => &next_game[2..],            
            };

            // Push the frame on the vector and compute again the remaining items
            vector.push(frame);
            remaining  = next_game.len();
        }

        Some(vector)
    }

    // Passing a slice of bowling game return the next frame
    fn chunk_to_frame (game: &[T]) -> Option<ImperativeFrame<T>> {

        use ImperativeFrame::{Strike, Spare, Open};
        
        match game {

            // Matching Strike
            [x, b1, b2, ..] if *x==10.into() => Some(Strike(*b1, *b2)),
            // Matching Spare
            [x, y, b1, ..]  if *x+*y==10.into() => Some(Spare(*b1)),
            // Matching Open
            [x, y, ..] if *x!=10.into() && *x+*y<10.into() => Some(Open(*x, *y)),
            // Otherwise return None
            _ => None,
        }
    }

    // The same function with the recursively or imperative implementation
    fn frame_list_to_score (game: &Vec<ImperativeFrame<T>>) -> Vec<T> {
        
        // Lets make a closure to keep the map beauty
        let mapper = | element: &ImperativeFrame<T> | -> T {
            
            // Match the variant and sum the points
            match element {
                ImperativeFrame::Strike(x,y) => (*x+*y)+10.into(),
                ImperativeFrame::Spare(x) => *x+10.into(),
                ImperativeFrame::Open(x,y) => *x+*y,
            }
        };

        // Iterate all array and translate from Frame<T> to T
        game.iter()
            .map(mapper)
            .collect()
    }
}

#[derive(Debug, Clone)]
enum RecursiveFrame<T> {
    Open(T, T),
    Spare(T),
    Strike(T, T),
}

impl<T> RecursiveFrame<T> 
    where T: 
        Copy +
        Clone +
        From<u8> +
        PartialEq + 
        PartialOrd + 
        std::iter::Sum +
        std::ops::Add<Output = T > {

    // The same function with the recursively or imperative implementation
    fn frame_list_to_score (game: &Vec<RecursiveFrame<T>>) -> Vec<T> {
        
        // Lets make a closure to keep the map beauty
        let mapper = | element: &RecursiveFrame<T> | -> T {
            
            // Match the variant and sum the points
            match element {
                RecursiveFrame::Strike(x,y) => (*x+*y)+10.into(),
                RecursiveFrame::Spare(x) => *x+10.into(),
                RecursiveFrame::Open(x,y) => *x+*y,
            }
        };

        // Iterate all array and translate from Frame<T> to T
        game.iter()
            .map(mapper)
            .collect()
    }

    // Recursive way to implement this
    fn to_frame(game: &[T]) -> Option<Vec<RecursiveFrame<T>>> {
        use RecursiveFrame::{Strike, Spare, Open};

        match game {
            
            // Strike in last position
            [x, b1, b2] if *x==10.into() => {
                Some(vec![Strike(*b1, *b2)])
            },
            // Strike in middle position
            [x, next@ ..] if *x==10.into() => {
                Some([&vec![Strike(*next.get(0)?, *next.get(1)?)], RecursiveFrame::to_frame(next)?.as_slice()].concat())
            },
    
            // Spare in last position
            [x, y, b1]  if *x+*y==10.into() => {
                Some(vec![Spare(*b1)])
            },
            // Spare in middle position
            [x, y, next@ ..]  if *x+*y==10.into() => {
                Some([&vec![Spare(*next.get(0)?)], RecursiveFrame::to_frame(next)?.as_slice()].concat())
            },
    
            // Open wherever its found
            [x, y, next@ ..] if *x<10.into() && *x+*y<10.into() => {
                Some([&vec![Open(*x, *y)], RecursiveFrame::to_frame(next)?.as_slice()].concat())
            },
    
            // When slice is empty return empty vec
            [] => Some(Vec::new()),
    
            // Otherwise something failed and we return None
            _ => None,
        }
    }
}


fn main() {
    let bowling_game: Vec<u32>= vec![1; 20];

    let frames_array = ImperativeFrame::to_frame(&bowling_game).expect("Bad bowling data!");
    let scores_array = ImperativeFrame::frame_list_to_score(&frames_array);
    let score = scores_array.iter().sum::<u32>();

    println!("\nImperative Way\n{:?}\n{:?}\n{:?}\n", frames_array, scores_array, score);

    let frames_array = RecursiveFrame::to_frame(&bowling_game).expect("Bad bowling data!");
    let scores_array = RecursiveFrame::frame_list_to_score(&frames_array);
    let score = scores_array.iter().sum::<u32>();

    println!("\nRecursive Way\n{:?}\n{:?}\n{:?}\n", frames_array, scores_array, score);
}

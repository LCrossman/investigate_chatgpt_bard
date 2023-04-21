**A Data Science View of ChatGPT and Bard

What if a Teacher asked the class to write a nonsense poem as a creative task for homework.  If a whole class of 35, uses ChatGPT or Bard to write a nonsense poem about a runcible spoon – would the teacher be able to tell?  In other words, is the output repetitive if asked to carry out a specific task several times.  Even the number of verses or a pattern in the length of the sentences could alert our teacher and the game could be up!

To test this out, I collected a set of 35 poems requesting both ChatGPT and Bard to “write a nonsense poem about a runcible spoon”, on several different days and times and saving the results to a text file.  So, let’s focus on the runcible spoon, a made-up word or so I thought.  It turned out ChatGPT knew more than me on that point.  "Runcible" far from being a made-up word turns out to be a description of a utensil somewhere between a spoon and a fork from the olden days (like a “spork” used in camping).  In different poems, the spoon was described as silly, odd, fancy, chic, wriggly and bright, of silver, of tin, of gold, had powers untold, with a handle made of silver, wood, jelly, shaped like a pair of dice, made of a rainbow’s light, made of a metal that nobody knows, glows in the dark like a bright red rose.
The spoon was many times described by ChatGPT as having a bowl as well as tines (like a fork), but Bard was more likely to describe the spoon as having a long handle.  The spoon went on many adventures and had many friends.  Whilst many poems had the spoon scooping and twirling, three ChatGPT poems and one Bard poem particularly stood out.  One involved a pink gorilla with a runcible hat, while a second and the Bard poem involved a runcible cat and one poem told an entire story finishing with “For it was not the spoon, that held the magic inside, But the heart of the owner, that made dreams come alive”.  
 
I compared the poems with the cosine similarity metric of the word frequencies as below.

```
fn cosine_similarity(val: &[f64], va: &[f64]) -> f64 {
    let dot: f64 = val.iter().copied().zip(va.iter().copied()).map(|(a,b)| a * b).sum();
    let sq1 = val.iter().map(|x| x * x).sum::<f64>();
    let sq2 = va.iter().map(|y| y * y).sum::<f64>();
    let prods = sq1.sqrt() * sq2.sqrt();
    if prods == 0.0 { 0.0 } else { dot/prods }
}  
```

And the results:  The Bard poems were more similar to each other in the cosine similarity metric than the ChatGPT poems, however, Bard wrote both the longest poem and the shortest.  In mean and median poem lengths, ChatGPT tended to write longer poems.  With a smaller difference in word range, the teacher might pick up on ChatGPT from that point of view, whilst Bard had a greater range of different layouts but more similar word frequencies.
In conclusion, I felt that I had a personal preference for the ChatGPT poems, although their more consistent layout compared to Bard poems could prove a way for our teacher to identify AI use.




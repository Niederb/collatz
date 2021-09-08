# collatz
Helper functions related to the Collatz conjecture (also know as the 3n + 1 problem or the 3n + 1 conjecture).
## Problem
Given a positive integer N do the following steps repeatedly:
- If the number is even, divide it by two.
- If the number is odd, triple it and add one.
The Collatz conjecture states that for every positive integer you will reach one in a finite number of steps.

## Examples
- Performance comparison: single threaded vs using rayon
- Plotting collatz steps in the console for a specified number

```
Please enter a positive integer
19
Collatz steps for number 19
Collatz steps [19, 58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
⡁           ⢰⡇                                                                              88.0
⠄           ⡎⢸                                                                             
⠂          ⢠⠃ ⡇                                                                            
⡁          ⡜  ⠸⡀                                                                           
⠄         ⢀⠇   ⢇                                                                           
⠂   ⣴     ⡸    ⠸⡀                                                                          
⡁  ⢰⠁⢣    ⡇     ⢇                     ⡴⡀                                                   
⠄ ⢀⠇ ⠈⢆  ⢸      ⠘⡄                   ⢰⠁⠱⡀                                                  
⠂ ⡜   ⠘⡄ ⡇       ⠘⡄                 ⢀⠇  ⠱⡀         ⡴⡀                                      
⡁⢰⠁    ⠸⣰⠁        ⠘⢄         ⡠⢆     ⡜    ⠱⡀       ⡜ ⠱⡀                                     
⢄⠇      ⠙          ⠈⢆       ⡰⠁ ⠱⡀  ⢰⠁     ⠱⡀     ⡜   ⠘⡄                                    
⡞                   ⠈⠢⢄    ⡔⠁   ⠈⢆⢀⠇       ⠑⢄   ⡜     ⠘⢄⡀                                  
⡁                      ⠑⢄ ⡜       ⠙          ⠑⢄⡜        ⠈⠢⢄         ⡠⢄⡀                    
⠄                        ⠙                     ⠁           ⠑⠤⢄⣀   ⡠⠊  ⠈⠑⠢⣀⡀                
⠂                                                              ⠉⠑⠊        ⠈⠉⠒⠤⠤⠤⠤⢄⣀⣀⣀⡀     
⠁                                                                                    ⠈      1.0
0.0                                                                                    21.0
```
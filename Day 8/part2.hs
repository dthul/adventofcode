increase ('"':xs) = 1 + increase xs
increase ('\\':xs) = 1 + increase xs
increase (_:xs) = increase xs
increase [] = 2

main = print . sum . map increase . lines =<< readFile "input.txt"

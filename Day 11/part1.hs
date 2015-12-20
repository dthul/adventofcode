import Data.List

input = "hepxcrrq"

incr :: Char -> (Char, Bool)
incr x
    | x < 'z' = (succ x, False)
    | otherwise = ('a', True)

incrl' :: [Char] -> ([Char], Bool)
incrl' (x:[]) = let (xi,c) = incr x in ([xi],c)
incrl' (x:xs)
    | c == True = let (xi, c) = incr x in (xi:xsi, c)
    | otherwise = (x:xsi,False)
    where (xsi, c) = incrl' xs

incrl :: [Char] -> [Char]
incrl = fst . incrl'

incrcheck :: [Char] -> Bool
incrcheck (x:y:z:xs)
    | succ x == y && succ y == z = True
    | otherwise = incrcheck $ y:z:xs
incrcheck _ = False

charcheck :: [Char] -> Bool
charcheck = not . any (\c -> c == 'i' || c == 'o' || c == 'l')

paircheck :: [Char] -> Bool
paircheck = (>1) . length . nub . map head . filter ((>1) . length) . group

pwcheck :: [Char] -> Bool
pwcheck pw = incrcheck pw && charcheck pw && paircheck pw

main = print . find pwcheck . tail $ iterate incrl input

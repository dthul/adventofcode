import Data.List

input = "1321131112"

lookAndSay = concat . map (\x -> (show . length $ x) ++ [head x]) . group

main = print . length $ iterate lookAndSay input !! 50

import Data.Aeson
import Data.HashMap.Strict (elems)
import Data.Maybe (fromJust)
import Data.Scientific (toBoundedInteger)
import Data.Text (pack)
import qualified Data.ByteString.Lazy.Char8 as B
import qualified Data.Vector as Vec

objectContainsRed :: Object -> Bool
objectContainsRed = any (== red) . elems
    where red = toJSON "red"

sumJSON :: Value -> Int
sumJSON (Object o)
    | objectContainsRed o = 0
    | otherwise = sum . map sumJSON $ elems o
sumJSON (Array a) = Vec.sum $ Vec.map sumJSON a
sumJSON (Number n) = fromJust $ toBoundedInteger n
sumJSON _ = 0

main = do
    input <- readFile "input.txt"
    print . sumJSON . fromJust . decode $ B.pack input

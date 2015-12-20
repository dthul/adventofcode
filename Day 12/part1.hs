import Data.Aeson
import Data.HashMap.Strict (elems)
import Data.Maybe (fromJust)
import Data.Scientific (toBoundedInteger)
import qualified Data.ByteString.Lazy.Char8 as B
import qualified Data.Vector as Vec

sumJSON :: Value -> Int
sumJSON (Object o) = sum . map sumJSON $ elems o
sumJSON (Array a) = Vec.sum $ Vec.map sumJSON a
sumJSON (Number n) = fromJust $ toBoundedInteger n
sumJSON _ = 0

main = do
    input <- readFile "input.txt"
    print . sumJSON . fromJust . decode $ B.pack input

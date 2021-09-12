import Data.List (intercalate)

data BST a = EmptyTree | BSTNode {
    val :: a
    , left :: BST a
    , right :: BST a
} deriving (Show, Read, Eq)

singleNode :: a -> BST a
singleNode x = BSTNode x EmptyTree EmptyTree

treeInsert :: (Ord a) => BST a -> a -> BST a
treeInsert EmptyTree x = singleNode x
treeInsert node x
    | x < val = BSTNode val (treeInsert left x) right
    | x > val = BSTNode val left (treeInsert right x)
    | otherwise = BSTNode x left right
    where BSTNode val left right = node

treePreorder :: (Ord a) => BST a -> [a]
treePreorder EmptyTree = []
treePreorder (BSTNode val left right) = val : treePreorder left ++ treePreorder right

treePostorder :: (Ord a) => BST a -> [a]
treePostorder EmptyTree = []
treePostorder (BSTNode val left right) = treePostorder left ++ treePostorder right ++ [val]

treeInorder :: (Ord a) => BST a -> [a]
treeInorder EmptyTree = []
treeInorder (BSTNode val left right) = treeInorder left ++ [val] ++ treeInorder right


drawLine id1 id2 = [show id1 ++ " -> " ++ show id2 ++ ";\n"]
labelNode label id = [show id ++ "[label=" ++ show label ++ "]\n"]

treeVisualizeConnections :: (Ord a, Eq a, Show a) => Int -> BST a -> [String]
treeVisualizeConnections id EmptyTree = labelNode "nil" id
treeVisualizeConnections id (BSTNode val left right) =
    labelNode val id
    ++
    drawLine id leftId
    ++
    drawLine id rightId
    ++
    treeVisualizeConnections (id * 2 + 1) left
    ++
    treeVisualizeConnections (id * 2 + 2) right
    where
        leftId = id * 2 + 1
        rightId = id * 2 + 2


treeVisualize :: (Ord a, Eq a, Show a) => BST a -> String
treeVisualize tree =
    "digraph graphname {\n"
    ++ intercalate "" (map ("  " ++) $ treeVisualizeConnections 0 tree)
    ++ "}"


main = do
    let values = [3, 5, 1, 2, 7, 9, 4, 6]

    let tree = foldl treeInsert EmptyTree values

    putStrLn $ treeVisualize tree

    return()
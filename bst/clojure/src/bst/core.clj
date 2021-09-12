(ns bst.core
  (:gen-class))

(defrecord BST [data left right])

(defn bst-add [bst value]
  (let [{:keys [data left right]} bst]
    (cond
      (= nil bst) (BST. value nil nil)
      (< value data) (BST. data (bst-add left value) right)
      (> value data) (BST. data left (bst-add right value))
      :else (BST. value left right))))

(defn bst-init [&values]
  (reduce (fn [bst elem] (bst-add bst elem)) nil &values)
)

(defn bst-size [bst]
  (let [{:keys [_ left right]} bst]
    (if (= nil bst) 0 (+ 1 (bst-size left) (bst-size right)))))

(defn bst-inorder [bst]
  (let [{:keys [data left right]} bst]
    (if (= nil bst)
      '()
      (concat
        (bst-inorder left)
        (list data)
        (bst-inorder right)))))

(defn bst-preorder [bst]
  (let [{:keys [data left right]} bst]
    (if (= nil bst)
      '()
      (concat
        (list data)
        (bst-preorder left)
        (bst-preorder right)))))

(defn bst-postorder [bst]
  (let [{:keys [data left right]} bst]
    (if (= nil bst)
      '()
      (concat
        (list data)
        (bst-postorder left)
        (bst-postorder right)))))


(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (def values '(10, 5, 3, 15, 12, 7))
  (def bst (bst-init values))

  (println "inorder: ")
  (run! println (bst-inorder bst))
  (println "\npreorder: ")
  (run! println (bst-preorder bst))
  (println "\npostorder: ")
  (run! println (bst-postorder bst))
  (println (str "\nsize: " (bst-size bst)))
  )

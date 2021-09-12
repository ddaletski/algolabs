(defproject bst "0.1.0"
  :description "Binary search tree implementation in clojure lang."
  :dependencies [[org.clojure/clojure "1.10.1"]]
  :main ^:skip-aot bst.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})

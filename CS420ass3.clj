;; CS420 
;; Assignment 3
;; Programmer: Josh Brown | 822455771
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; **Question 1
;; We can use a map to describe an item on a restaurant bill. For example: 
;; {:name “Green Tea Ice Cream” :price 2.5 :quantity 2}.
;; We can represent the bill as a vector of maps.
;; Write a Clojure function bill-total whose one argument is a vector of such maps and returns the total of the bill.
;; For example: 

;;     (def bill [{:name “Green Tea Ice Cream” :price 2.5 :quantity 2}
;;                {:price 1.0 :name “Sticky Rice” :quantity 1}])
;;     (bill-total bill) returns 6.0
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def bill [{:name “Green Tea Ice Cream” :price 2.5 :quantity 2}
                {:price 1.0 :name “Sticky Rice” :quantity 1}])

(defn billfunc [bill]
  (* (:price bill) (:quantity bill)))

(defn bill-total [bill]
  (reduce + (map billfunc bill)))

(bill-total bill)

;; **Question 2
;; Often people will order additional times.  Write a function add-to-bill that accepts two arguments. The ﬁrst is a 
;; bill as above. The second argument is a vector of additional items. The method returns a new bill with the additional 
;; items. For example: 

;; (def items [{:price 2.1 :name “Mango” :quantity 1} { :quantity 1:price 1.0 :name “Sticky Rice” }

;; (add-to-bill bill items) returns [{:name “Green Tea Ice Cream” :price 2.5 :quantity 2}
;;                                   {:price 1.0 :name “Sticky Rice” :quantity 2}
;;                                   {:price 2.1 :name “Mango” :quantity 1}]
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def bill [{:name "Green Tea Ice Cream" :price 2.5 :quantity 2},
           {:price 1.0 :name "Sticky Rice" :quantity 1}
           {:price 3.7 :name "Mango Tea" :quantity 1}])

(defn add-to-bill [bill item]
  (conj bill item))

(def newitem {:price 4.0 :name "Orange Chicken" :quantity 2})

(add-to-bill bill newitem)

;; **Question 3
;; Write a function, divisors, with one argument, a positive integer. 
;; The function returns a sequence of the divisors of N.
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn divisors [x]
  (let [r (range 1 (+ (/ x 2) 1))] 
    (for [i r
          :when (= (mod x i) 0)]
      i)))

;; **Question 4
;; An abundant number is an integer for which the sum of its proper divisors is greater than twice the number.
;; For example, 12 is an abundant number as its divisors are 1, 2, 3, 4, 6, and 12, totaling 28. Write a function, 
;; Write a function, abundance, that has one argument, an integer, and returns the sum of the proper divisors of the  
;; number minus the number itself. For example, (abundance 12) returns 4.
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn abundance [x]
  (- (reduce + (divisors x)) x))

(abundance 6)

;; **Question 5
;; Find all the abundant numbers less than 300.
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(loop [x 1]
  (when (< x 301)
    (if (> (abundance x) 0)
      (println x))
    (recur (+ x 1))
    ))

;; **Question 6
;; Write a function, pattern-count, with two arguments. The ﬁrst argument is a string, let's call it text, and the 
;; second argument is also a string; call it a pattern. The function pattern count returns the number of times the 
;; pattern occurs in the text. For example

;;     (pattern-count “abababa” “aba”) returns 3
;;     (pattern-count “aaaaa” “aa”) returns 4
;;     (pattern-count “Abcde” “abc”) returns 0
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn pattern-count [text pattern]
  (count
   (re-seq
    (re-pattern pattern)
    text)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;



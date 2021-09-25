(define (fib x)
  (if (<= x 1) x 
	(+ (fib (- x 1 )) (fib (- x 2)))))
(print! "fib(10)= ")
(println! (fib 10))

(define (fact x)
  (if (<= x 1 ) 1 
	(* x (fact (- x 1)))))

(print! "fact(10)= ")
(println! (fact 10))

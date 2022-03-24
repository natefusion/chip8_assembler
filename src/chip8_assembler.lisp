;;#!/usr/bin/sbcl --script
;;(load "/home/nathan/quicklisp/setup.lisp")
;;(ql:quickload :trivia)
;;(proclaim '(optimize (speed 3) (safety 0) (debug 0)))
;;(use-package :trivia)

(defun chip8-setup ()
  (progn (ql:quickload :trivia)
         (use-package :trivia)))
         

(defun parse (l)
  (eval (read-from-string (concatenate 'string "'(" l ")"))))

(defun flatten (l)
  (apply #'concatenate 'string l))

(defun make-sexp (s)
  (let ((trim (string-trim " " s)))
    (if (and (string/= (subseq trim 0 1) "(")
             (string/= (subseq trim 0 1) ")"))
        (concatenate 'string "(" trim ") ")
        (concatenate 'string trim " "))))

(defun remove-blank (l)
  (remove-if
   (lambda (x) (or (string= (string-trim " " x) "")
                   (string= (subseq (string-trim " " x) 0 1) ";")))
     l))

(defun tokenize (x)
  (flatten
    (mapcar #'make-sexp
            (remove-blank (uiop:read-file-lines x)))))

(defun make-env ()
  (let ((ht (make-hash-table)))
    (setf (gethash 'EQ ht) #'emit-op)
    (setf (gethash 'NEQ ht) #'emit-op)
    (setf (gethash 'SET ht) #'emit-op)
    (setf (gethash 'ADD ht) #'emit-op)
    (setf (gethash 'OR ht) #'emit-op)
    (setf (gethash 'AND ht) #'emit-op)
    (setf (gethash 'XOR ht) #'emit-op)
    (setf (gethash 'SUB ht) #'emit-op)
    (setf (gethash 'SHR ht) #'emit-op)
    (setf (gethash 'SUBR ht) #'emit-op)
    (setf (gethash 'SHL ht) #'emit-op)
    (setf (gethash 'RAND ht) #'emit-op)
    (setf (gethash 'DRAW ht) #'emit-op)
    (setf (gethash 'BCD ht) #'emit-op)
    (setf (gethash 'WRITE ht) #'emit-op)
    (setf (gethash 'READ ht) #'emit-op)
    (setf (gethash 'CLEAR ht) #'emit-op)
    (setf (gethash 'RET ht) #'emit-op)
    (setf (gethash 'CALL ht) #'emit-op)
    (setf (gethash 'JUMP ht) #'emit-op)
    (setf (gethash 'JUMP0 ht) #'emit-op)
    ;; these math functions dont work because emit-op acts weird
    ;; maybe make a function for each instruction that just runs emit-op
    (setf (gethash '+ ht) #'+)
    (setf (gethash '- ht) #'-)
    (setf (gethash '* ht) #'*)
    (setf (gethash '/ ht) #'/)

    (setf (gethash 'BREAK ht) (lambda () '(BREAK)))

    (list #x200 ht)))

(defun chip8-type (exp)
  (cond
    ((v? exp) 'V)
    ((builtin-var? exp) exp)
    (t 'N)))

(defun combine-op (args shell&info)
  (let ((shell (car shell&info))
        (info (cadr shell&info)))
    (loop :for x :in args
          :for i :from 0
          :do (let ((shift (logand (ash info (* i -4)) #xF)))
                (setf shell (logior shell (ash x shift))))
          :finally (return (list (ash (logand shell #xFF00) -8)
                                 (logand shell #xFF))))))

(defun emit-op (proc args env)
  (progn
    (incf (first env) 2)

    (let ((stripped-args
            (remove-if #'builtin-var? (chip8-eval-args-partial args env :eval-v t))))
      (if (not (null (remove-if #'numberp stripped-args)))
          (list (cons proc args))
          (combine-op
           stripped-args
     
           (match (append (list proc) (mapcar #'chip8-type args))
             ('(EQ V V) '(#x9000 #x48))
             ('(EQ V N) '(#x4000 #x8))
             ('(EQ V KEY) '(#xE0A1 #x8))
             
             ('(NEQ V KEY) '(#xE09E Ex81))
             ('(NEQ V V) '(#x5000 #x48))
             ('(NEQ V N) '(#x3000 #x8))
             
             ('(SET V N) '(#x6000 #x8))
             ('(SET V V) '(#x8000 #x48))
             ('(SET I N) '(#xA000 #x0))
             ('(SET V DT) '(#xF007 #x8))
             ('(SET DT V) '(#xF015 #x8))
             ('(SET V ST) '(#xF018 #x8))
             ('(SET I V) '(#xF029 #x8))
             ('(SET V KEY) '(#xF00A #x8))
             
             ('(ADD V N) '(#x7000 #x8))
             ('(ADD V V) '(#x8004 #x48))
             ('(ADD I V) '(#xF01E #x8))
             
             ('(OR V V) '(#x8001 #x48))
             ('(AND V V) '(#x8002 #x48))
             ('(XOR V V) '(#x8003 #x48))
             ('(SUB V V) '(#x8005 #x48))
             ('(SHR V V) '(#x8006 #x48))
             ('(SUBR V V) '(#x8007 #x48))
             ('(SHL V V) '(#x800E #x48))
             
             ('(RAND V N) '(#xC000 #x8))
             ('(DRAW V V N) '(#xD000 #x48))
             
             ('(BCD V) '(#xF033 #x8))
             ('(WRITE V) '(#xF055 #x8))
             ('(READ V) '(#xF065 #x8))
             
             ('(CLEAR) '(#x00E0 #x0))
             ('(RET) '(#x00EE #x0))
             ('(CALL N) '(#x2000 #x0))
             ('(JUMP N) '(#x1000 #x0))
             ('(JUMP0 N) '(#xB000 #x0))
             (_ '(0 0))))))))
    
(defun ins? (exp)
  (and (listp exp)
       (match (first exp)
         ('EQ 't) ('NEQ 't) ('SET 't) ('ADD 't)
         ('OR 't) ('AND 't) ('XOR 't) ('SUB 't)
         ('SHR 't) ('SUBR 't) ('SHL 't) ('RAND 't)
         ('DRAW 't) ('BCD 't) ('WRITE 't) ('READ 't)
         ('CLEAR 't) ('RET 't) ('CALL 't) ('JUMP 't)
         ('JUMP0 't)
         (_ nil))))

(defun chip8-eval-args-partial (args env &key eval-v)
  (mapcar (lambda (x)
            (if (and (v? x) eval-v)
                (chip8-eval-v? x)
                (chip8-eval x env)))
          args))

(defun chip8-eval-v? (exp)
  (match exp
    ('V0 0) ('V1 1) ('V2 2) ('V3 3)
    ('V4 4) ('V5 5) ('V6 6) ('V7 7)
    ('V8 8) ('V9 9) ('VA #xA) ('VB #xB)
    ('VC #xC) ('VD #xD) ('VE #xE) ('VF #xF)
    (t nil)))

(defun v? (exp)
  (not (null (chip8-eval-v? exp))))

(defun builtin-var? (exp)
  (or (eq exp 'KEY)
      (eq exp 'ST)
      (eq exp 'DT)
      (eq exp 'I)))

(defun self-evaluating? (exp)
  (and (not (listp exp))
       (or (numberp exp)
           (builtin-var? exp)
           (v? exp))))

(defun def? (exp)
  (and (listp exp)
       (eq (first exp) 'DEF)))
  
(defun chip8-eval-def (exp env)
  (progn
    (setf (gethash (cadr exp) (cadr env)) (caddr exp))
    nil))
       
(defun label? (exp)
  (and (listp exp)
       (eq (first exp) 'LAB)))

(defun chip8-eval-label (exp env)
  (progn
    (setf (gethash (cadr exp) (cadr env)) (car env))
    nil))

(defun loop? (exp)
  (and (listp exp)
       (eq (first exp) 'LOOP)))

(defun chip8-eval-loop (exp env)
  (let ((label (list (first env)))
        (loop-body (chip8-eval-file (rest exp) env)))
   (append
     (loop :for x :in loop-body
           :if (eq x 'BREAK)
             :append (emit-op 'JUMP (list (+ 4 (car env))) env)
           :else
             :collect x)
     (emit-op 'JUMP label env))))
  
(defun var? (exp)
  (and (not (application? exp))
       (not (self-evaluating? exp))))

(defun chip8-eval-var (exp env)
  (let ((var (gethash exp (cadr env))))
    (if var var exp)))

(defun application? (exp)
  (and (not (null exp))
       (listp exp)
       (not (def? exp))
       (not (label? exp))))

(defun include? (exp)
  (and (listp exp)
       (eq (first exp) 'INCLUDE)))

(defun chip8-eval-include (exp env)
  (progn
    (incf (car env) (length (remove-if-not #'numberp exp)))
    (chip8-eval-args-partial (rest exp) env)))

(defun process-labels (exps env)
  "Flattens list and processes unresolved labels"
  (cond
    ((null exps) nil)
    ((atom exps) (list exps))
    (t (mapcan (lambda (x) (process-labels (chip8-eval x env) env)) exps))))

(defun rotate-main (exps offset)
  "Ensures that the code below 'lab main' is always at the beginning"
    (append (nthcdr offset exps)
            (reverse (nthcdr (- (length exps) offset)
                             (reverse exps)))))

(defun chip8-eval-top (exps env)
  (let ((program (process-labels (chip8-eval-file exps env) env)))
    (let ((main-label (gethash 'main (cadr env))))
      (if main-label
          (rotate-main program (- main-label #x200))
          "please add a main label"))))

(defun chip8-eval-file (exps env)
  (cond
    ((null exps) nil)
    (t (append (chip8-eval (car exps) env)
               (chip8-eval-file (rest exps) env)))))

(defun chip8-err (exp)
  (format t "You typed: ~a~%That was bad~%" exp))

(defun chip8-eval (exp env)
  (cond ((self-evaluating? exp) exp)
        ((def? exp) (chip8-eval-def exp env))
        ((label? exp) (chip8-eval-label exp env))
        ((var? exp) (chip8-eval-var exp env))
        ((loop? exp) (chip8-eval-loop exp env))
        ((include? exp) (chip8-eval-include exp env))
        ((ins? exp)
         (funcall (chip8-eval (first exp) env)
                  (first exp)
                  (chip8-eval-args-partial (rest exp) env) env))
        ((application? exp)
         (apply (chip8-eval (first exp) env)
                (chip8-eval-args-partial (rest exp) env :eval-v t)))
        (t (chip8-err exp))))

(defun chip8-compile (file)
  (chip8-eval-top
   (parse (tokenize file))
   (make-env)))

(defun chip8-write (bytes filename)
  (with-open-file (f filename
                     :direction :output
                     :if-exists :supersede
                     :if-does-not-exist :create
                     :element-type 'unsigned-byte)
    (mapcar (lambda (x) (write-byte x f)) bytes)))

;;(defun main ()
;;  (cond
;;    ((>= (length *posix-argv*) 2)
;;     (format t "~a~%"
;;     (chip8-eval-top
;;       (parse (tokenize (cadr *posix-argv*)))
;;       (make-env))))
;;    (t nil)))

;;(main)

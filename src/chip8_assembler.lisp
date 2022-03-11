(defparameter *filename*
  "/home/nathan/Documents/programming/chip8_assembler/src/default.ch8")

(defun parse (l)
  (eval (read-from-string (concatenate 'string "'(" l ")"))))

(defun flatten (l)
  (apply #'concatenate 'string l))

(defun flatten-list (structure)
  (cond ((null structure) nil)
        ((atom structure) (list structure))
        (t (mapcan #'flatten-list structure))))

(defun make-sexp (s)
  (let ((trim (string-trim " " s)))
    (if (and (string/= (subseq trim 0 1) "(") (string/= (subseq trim 0 1) ")"))
        (concatenate 'string "(" trim ") ")
        (concatenate 'string trim " "))))

(defun remove-blank (l)
  (remove-if
   (lambda (x) (string= x "")) l))

(defparameter *file*
  (parse
   (flatten
    (mapcar #'make-sexp
            (remove-blank (uiop:read-file-lines *filename*))))))


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
    (setf (gethash 'RETURN ht) #'emit-op)
    (setf (gethash 'CALL ht) #'emit-op)
    (setf (gethash 'JUMP ht) #'emit-op)
    (setf (gethash 'JUMP0 ht) #'emit-op)
    (setf (gethash '+ ht) #'+)
    (setf (gethash '- ht) #'-)
    (setf (gethash '* ht) #'*)
    (setf (gethash '/ ht) #'/)
    (list #x202 ht)))

(defun chip8-type (exp)
  (cond
    ((v? exp) 'V)
    ((builtin-var? exp) exp)
    ((numberp exp) 'N)
    (t nil)))

(defun combine-op (args shell&info)
  (let ((shell (car shell&info))
        (info (cadr shell&info)))
    (loop :for x :in args
          :for i :from 0
          :do (let ((shift (logand (ash info (* i -4)) #xF)))
                (setf shell (logior shell (ash x shift))))
          :finally (return (list (ash (logand shell #xFF00) -8)
                                 (logand shell #xFF))))))

(ql:quickload :trivia)
(use-package :trivia)
(defun emit-op (proc args env)
  (progn
    (incf (first env) 2)
    (combine-op
     (remove-if #'builtin-var?
                    (chip8-eval-args-partial args env :eval-v t))
     (match (append (list proc) (mapcar #'chip8-type args))
       ('(EQ V V) '(#x9000 #x48))
       ((or '(EQ V N)
            '(EQ N V))
        '(#x4000 #x8))
       ((or '(EQ V KEY)
            '(EQ KEY V))
        '(#xE0A1 #x8))
       
       ((or '(NEQ V KEY)
            '(NEQ KEY V))
        '(#xE09E Ex81))
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
       ('(RETURN) '(#x00EE #x0))
       ('(CALL N) '(#x2000 #x0))
       ('(JUMP N) '(#x1000 #x0))
       ('(JUMP0 N) '(#xB000 #x0))
       (_ '(0 0))))))

(defun chip8-eval-args-partial (args env &key eval-v)
  (mapcar (lambda (x)
            (if (and (v? x) eval-v) (chip8-eval-v? x) (chip8-eval x env)))
          args))

(defun chip8-eval-v? (exp)
  (cond ((eq exp 'V0) 0)
        ((eq exp 'V1) 1)
        ((eq exp 'V2) 2)
        ((eq exp 'V3) 3)
        ((eq exp 'V4) 4)
        ((eq exp 'V5) 5)
        ((eq exp 'V6) 6)
        ((eq exp 'V7) 7)
        ((eq exp 'V8) 8)
        ((eq exp 'V9) 9)
        ((eq exp 'VA) #xA)
        ((eq exp 'VB) #xB)
        ((eq exp 'VC) #xC)
        ((eq exp 'VD) #xD)
        ((eq exp 'VE) #xE)
        ((eq exp 'VF) #xF)
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
       (eq (first exp) 'DEF)
       (= (length exp) 3)))

(defun chip8-eval-def (exp env)
  (progn
    (setf (gethash (cadr exp) (cadr env)) (caddr exp))
    nil))
       
(defun label? (exp)
  (and (listp exp)
       (eq (first exp) 'DEF)
       (= (length exp) 2)))

(defun chip8-eval-label (exp env)
  (progn
    (setf (gethash (cadr exp) (cadr env)) (car env))
    nil))

(defun loop? (exp)
  (and (listp exp)
       (eq (first exp) 'LOOP)))

(defun chip8-eval-loop (exp env)
  (let ((label (list (first env))))
    (list (chip8-eval-file (rest exp) env)
          (emit-op 'JUMP label env))))

(defun var? (exp)
  (and (not (application? exp))
       (not (builtin-var? exp))))

(defun application? (exp)
  (and (listp exp)
       (not (def? exp))
       (not (label? exp))))

(defun include? (exp)
  (and (listp exp)
       (numberp (first exp))))

(defun chip8-eval-top (exps env)
  (let ((program (chip8-eval-file exps env)))
    (let ((main-label (gethash 'main (cadr env))))
      (if main-label
          (append (emit-op 'JUMP (list main-label) env) program)
          "please add a main label"))))

(defun chip8-eval-file (exps env)
  (flatten-list
   (remove-if #'null
              (mapcar (lambda (x) (chip8-eval x env)) exps))))

(defun chip8-eval (exp env)
  (cond ((self-evaluating? exp) exp)
        ((def? exp) (chip8-eval-def exp env))
        ((label? exp) (chip8-eval-label exp env))
        ((var? exp) (gethash exp (cadr env)))
        ((loop? exp) (chip8-eval-loop exp env))
        ((include? exp) (progn (incf (car env) (length exp)) exp))
        ((application? exp)
         (funcall (chip8-eval (first exp) env)
                  (first exp)
                  (chip8-eval-args-partial (rest exp) env) env))
        (t "uh oh")))


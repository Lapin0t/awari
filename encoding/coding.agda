open import Data.Nat
  renaming (_∸_ to _-_)
open import Data.Bool
open import Data.Vec
open import Relation.Nullary
open import Relation.Binary.PropositionalEquality


Awari : ℕ → Set
Awari n = Vec ℕ n


binom : ℕ → ℕ → ℕ
binom _ 0 = 0
binom 0 _ = 1
binom (suc k) (suc n) = binom k n + binom (suc k) n


{-binom-inv : ℕ → ℕ → ℕ
binom-inv k b = aux k b 1 k
  where
  aux : ℕ → ℕ → ℕ → ℕ → ℕ
  aux k b x n with x ≤? b
  ...         | no _ = n - 1
  ...         | yes _ = aux k b (x + binom k n) (suc n)-}


-- set-theory encoding of lists (list2set)
encode-vec : ∀ {n} → Vec ℕ n → Vec ℕ n
encode-vec [] = []
encode-vec (x ∷ xs) = aux xs x
  where
  aux : ∀ {n} → Vec ℕ n → ℕ → Vec ℕ (suc n)
  aux [] c = c ∷ []
  aux (x ∷ xs) c = c ∷ aux xs (c + x + 1)


-- set theory decoding of lists (set2list)
decode-vec : ∀ {n} → Vec ℕ n → Vec ℕ n
decode-vec [] = []
decode-vec (x ∷ xs) = x ∷ aux xs x
  where
  aux : ∀ {n} → Vec ℕ n → ℕ → Vec ℕ n
  aux [] _ = []
  aux (x ∷ xs) c = (x - c - 1) ∷ aux xs x


--enc-vec-bij : ∀ {n} → (v : Vec ℕ n) → (encode-vec (decode-vec v) ≡ v)
--enc-vec-bij _ = {! !}


encode-aux : {n : ℕ} → Awari n → ℕ → ℕ → ℕ
encode-aux {_} [] a _ = a
encode-aux {n} (x ∷ xs) a c = encode-aux xs a' c'
  where
  c' = c + x
  a' = a + binom n (c' + n - 1)

encode : {n : ℕ} → Awari n → ℕ
encode board = encode-aux board 0 0



method main() {
    assume forall qvar: Int ::
      let alias == (qvar) in
        f_get(alias) == f_clamp() // Here
    assert f_get(10) == 0
}

function f_get(idx: Int): Int


function f_clamp(): Int
  ensures result == 0
  
  
  

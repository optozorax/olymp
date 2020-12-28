fn fst<T1, T2>((t1, _): (T1, T2)) -> T1 { t1 }

fn snd<T1, T2>((_, t2): (T1, T2)) -> T2 { t2 }
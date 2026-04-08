
pub struct Vec<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct HashMap<K, V> {
    _marker: std::marker::PhantomData<(K, V)>,
}


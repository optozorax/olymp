#[derive(Debug, Clone)]
struct PushMap<K, V>(pub BTreeMap<K, Vec<V>>);

impl<K: Ord, V> FromIterator<(K, V)> for PushMap<K, V> {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> PushMap<K, V> {
		let mut result = BTreeMap::new();
		for (k, v) in iter.into_iter() {
			result.entry(k).or_insert_with(Vec::new).push(v);
		}
		PushMap(result)
	}
}

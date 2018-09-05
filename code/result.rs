items.map(|v| may_fail(v))
     .collect::<Result<Vec<_>, _>>()

use tempfile::tempdir;

use crate::db::Database;
use crate::models::Item;

#[test]
fn db_init_creates_tables() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("test.sqlite");
    let db = Database::new(&path).expect("db init");
    let items = db.list_items(10, true).expect("list items");
    assert!(items.is_empty());
}

#[test]
fn insert_and_list_orders_items() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("test.sqlite");
    let db = Database::new(&path).expect("db init");

    let item1 = Item {
        id: "1".into(),
        created_at: 1,
        item_type: "text".into(),
        title: None,
        payload: "a".into(),
        thumb_path: None,
        pinned: 0,
    };
    let item2 = Item {
        id: "2".into(),
        created_at: 2,
        item_type: "text".into(),
        title: None,
        payload: "b".into(),
        thumb_path: None,
        pinned: 1,
    };

    db.insert_item(&item1).expect("insert item1");
    db.insert_item(&item2).expect("insert item2");

    let items = db.list_items(10, true).expect("list items");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].id, "2");
    assert_eq!(items[1].id, "1");
}

#[test]
fn delete_removes_item() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("test.sqlite");
    let db = Database::new(&path).expect("db init");

    let item = Item {
        id: "1".into(),
        created_at: 1,
        item_type: "text".into(),
        title: None,
        payload: "a".into(),
        thumb_path: None,
        pinned: 0,
    };
    db.insert_item(&item).expect("insert item");
    db.delete_item("1").expect("delete item");

    let items = db.list_items(10, true).expect("list items");
    assert!(items.is_empty());
}

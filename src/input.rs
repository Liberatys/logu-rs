pub enum EntryType{
    INPUT,
    OUTPUT,
}


pub struct Entry{
    name: String,
    id: i32,
    entryType: EntryType,
}

impl Entry{

    pub fn new(name: String, id: i32, entryType: EntryType) -> Entry
    {
        let entry = Entry{
            name,
            id,
            entryType
        };
        return entry;
    }
}

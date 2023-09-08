use {
    crate::{connect, fork::Fork, Client, Result, Sql, Transaction, BACKGROUND_LINKS},
    doublets::{
        data::LinkType,
        mem::{Alloc, FileMapped},
        split::{self, DataPart, IndexPart},
        unit::{self, LinkPart},
        Doublets,
    },
    std::alloc::Global,
    tokio::runtime::Runtime,
};

pub trait Benched: Sized {
    type Builder<'a>;

    fn setup(builder: Self::Builder<'_>) -> Result<Self>;

    fn drop_storage(&mut self) -> Result<()>;

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }
}

impl<T: LinkType> Benched for unit::Store<T, FileMapped<LinkPart<T>>> {
    type Builder<'a> = FileMapped<LinkPart<T>>;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let mut links = Self::new(builder)?;
        for _ in 0..BACKGROUND_LINKS {
            links.create_point()?;
        }
        Ok(links)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> Benched for unit::Store<T, Alloc<LinkPart<T>, Global>> {
    type Builder<'a> = Global;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let storage = Alloc::new(builder);
        let mut links = Self::new(storage)?;
        for _ in 0..BACKGROUND_LINKS {
            links.create_point()?;
        }
        Ok(links)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> Benched for split::Store<T, FileMapped<DataPart<T>>, FileMapped<IndexPart<T>>> {
    type Builder<'a> = (FileMapped<DataPart<T>>, FileMapped<IndexPart<T>>);

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let (data, index) = builder;
        let mut links = Self::new(data, index)?;
        for _ in 0..BACKGROUND_LINKS {
            links.create_point()?;
        }
        Ok(links)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> Benched
    for split::Store<T, Alloc<DataPart<T>, Global>, Alloc<IndexPart<T>, Global>>
{
    type Builder<'a> = Global;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let (data, index) = (Alloc::new(builder), Alloc::new(builder));
        let mut links = Self::new(data, index)?;
        for _ in 0..BACKGROUND_LINKS {
            links.create_point()?;
        }
        Ok(links)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> Benched for Client<T> {
    type Builder<'a> = &'a Runtime;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let mut client = builder.block_on(connect())?;
        for _ in 0..BACKGROUND_LINKS {
            client.create_point()?;
        }
        Ok(client)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.drop_table()
    }

    fn fork(&mut self) -> Fork<Self> {
        let _ = self.create_table();
        Fork::new(self)
    }
}

impl<'a, T: LinkType> Benched for Transaction<'a, T> {
    type Builder<'b> = &'a mut Client<T>;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let mut transaction = builder.transaction().unwrap();
        transaction.create_table().unwrap();
        for _ in 0..BACKGROUND_LINKS {
            transaction.create_point().unwrap();
        }
        Ok(transaction)
    }

    fn drop_storage(&mut self) -> Result<()> {
        self.drop_table()
    }

    fn fork(&mut self) -> Fork<Self> {
        let _ = self.create_table();
        Fork::new(self)
    }
}

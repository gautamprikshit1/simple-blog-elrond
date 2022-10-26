#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[endpoint(createPost)]
    fn create_post(&self, title: ManagedBuffer, content: ManagedBuffer) {
        let post = BlogPost {
            blog_id: self.blog_posts().len() as u32 + 1,
            upvotes: 0,
            title,
            author: self.blockchain().get_caller(),
            content,
        };
        self.blog_posts().push(&post);
    }

    #[endpoint(editPost)]
    fn edit_post(
        &self,
        id: u32,
        upvote: bool,
        title: OptionalValue<ManagedBuffer>,
        content: OptionalValue<ManagedBuffer>,
    ) {
        let blog_post = self.blog_posts().get(id as usize);
        let post_upvotes = if upvote {
            blog_post.upvotes + 1
        } else {
            blog_post.upvotes
        };
        let updated_post = BlogPost {
            blog_id: id,
            title: OptionalValue::into_option(title).unwrap_or(blog_post.title),
            author: blog_post.author,
            upvotes: post_upvotes,
            content: OptionalValue::into_option(content).unwrap_or(blog_post.content),
        };
        self.blog_posts().set(id as usize, &updated_post);
    }

    #[endpoint(deletePost)]
    fn delete_post(&self, id: u32) {
        self.blog_posts().clear_entry(id as usize);
    }

    #[view(getBlogPosts)]
    #[storage_mapper("blogPosts")]
    fn blog_posts(&self) -> VecMapper<BlogPost<Self::Api>>;
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct BlogPost<M: ManagedTypeApi> {
    pub blog_id: u32,
    pub upvotes: u32,
    pub title: ManagedBuffer<M>,
    pub author: ManagedAddress<M>,
    pub content: ManagedBuffer<M>,
}

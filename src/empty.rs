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
            blog_id: self.blog_posts().len() + 1usize,
            upvotes: 0u32,
            title,
            author: self.blockchain().get_caller(),
            content,
        };
        self.blog_posts().push(&post);
    }

    #[endpoint(editPost)]
    fn edit_post(
        &self,
        id: usize,
        upvote: bool,
        title: OptionalValue<ManagedBuffer>,
        content: OptionalValue<ManagedBuffer>,
    ) {
	let blog_post_mapper = self.blog_posts();

	require!(blog_post_mapper.item_is_empty(id) == false, "ID not found");

        let blog_post = blog_post_mapper.get(id);
        let post_upvotes = if upvote {
            blog_post.upvotes + 1u32
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
        blog_post_mapper.set(id, &updated_post);
    }

    #[endpoint(deletePost)]
    fn delete_post(&self, id: usize) {
        self.blog_posts().clear_entry(id);
    }

    #[view(getBlogPosts)]
    #[storage_mapper("blogPosts")]
    fn blog_posts(&self) -> VecMapper<BlogPost<Self::Api>>;
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct BlogPost<M: ManagedTypeApi> {
    pub blog_id: usize,
    pub upvotes: u32,
    pub title: ManagedBuffer<M>,
    pub author: ManagedAddress<M>,
    pub content: ManagedBuffer<M>,
}

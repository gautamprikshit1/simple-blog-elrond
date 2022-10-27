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
	let blog_id = self.blog_posts().len() + 1usize;
	let author = self.blockchain().get_caller();
	let time = self.blockchain().get_block_timestamp();

        let post = BlogPost {
            blog_id,
            upvotes: 0u32,
            title,
            author,
            content,
	    time
        };
        self.blog_posts().push(&post);
	self.is_published(blog_id).set(false);
    }

    #[endpoint(editPost)]
    fn edit_post(
        &self,
        id: usize,
        upvote: bool,
        title: OptionalValue<ManagedBuffer>,
        content: OptionalValue<ManagedBuffer>,
    ) {
	let caller = self.blockchain().get_caller();

	let blog_post_mapper = self.blog_posts();

	match self.status(id){
	Status::NotExist => sc_panic!("ID not found"),
	Status::Published => sc_panic!("Unable edit Published post"),
	Status::Private => {

      		let blog_post = blog_post_mapper.get(id);
       		let post_upvotes = if upvote {
           	blog_post.upvotes + 1u32
       		} else {
           		blog_post.upvotes
       			};
		let author = blog_post.author;
		let time = self.blockchain().get_block_timestamp();

		require!(caller == author, "You are not author of this post");

       		let updated_post = BlogPost {
           		blog_id: id,
           		title: OptionalValue::into_option(title).unwrap_or(blog_post.title),
           		author: blog_post.author,
           		upvotes: post_upvotes,
           		content: OptionalValue::into_option(content).unwrap_or(blog_post.content),
	   		time
       		};
       		blog_post_mapper.set(id, &updated_post);
	}
    }

    #[endpoint(deletePost)]
    fn delete_post(&self, id: usize) {
	let caller = self.blockchain().get_caller();
	let blog_post_mapper = self.blog_posts();

	match self.status(id){
	Status::NotExist => sc_panic!("ID not found"),
	Status::Published => sc_panic!("Unable edit Published post"),
	Status::Private => {

		let blog_post = blog_post_mapper.get(id);
		let author = blog_post.author;

		require!(caller == author, "You are not author of this post");

       		self.blog_posts().clear_entry(id);
	}
    }

    #[endpoint(commentPost)]
    fn comment_post(&self, id: usize, comment: ManagedBuffer){
	let user = self.blockchain().get_caller();
	let comment_time = self.blockchain().get_block_time();
	match self.status(id){
		Status::NotExist => sc_panic!("ID not found"),
		Status::Private => sc_panic!("Unable to comment on private post"),
		Status::Published => {
			let blog_post_mapper = self.blog_posts();

			let post_comment = PostComment {
				user,
				comment,
				comment_time
			};

		self.post_comments(id).push_back(post_comment);
		}
     }
     
     #[endpoint(publishPost)]
     fn publish_post(&self, id: usize){
	let user = self.blockchain().get_caller();
	let blog_post_mapper = self.blog_posts();
	match self.status(id){
		Status::NotExist => sc_panic!("ID not found"),
		Status::Published => sc_panic!("Already published"),
		Status::Private => {

	let blog_post = blog_post_mapper.get(id);
	let author = blog_post.author;

	require!(caller == author, "You are not author of this post");
	self.is_published(id).set(true);
	self.published_posts().set(id, &blog_post);
}
     }

    #[view(getBlogPosts)]
    #[storage_mapper("blogPosts")]
    fn blog_posts(&self) -> VecMapper<BlogPost<Self::Api>>;

    #[view(getPostComments)]
    #[storage_mapper("postComments")]
    fn post_comments(&self, id: usize) -> LinkedListMapper<PostComment<Self::Api>>;

    #[view(getPublishedPosts)]
    #[storage_mapper("publishedPosts")]
    fn published_posts(&self) -> VecMapper<BlogPost<Self::Api>>;

    #[view(getPublishedState)]
    #[storage_mapper("publishedState")]
    fn is_published(&self, id: usize) -> SingleValueMapper<bool>;


    #[view(proposalStatus)]
    fn status(&self, id: usize) -> Status {
	let blog_post_state = self.blog_posts().item_is_empty(id);
	let is_published = self.is_published(id).get();

	if blog_post_state == true{
		Status::NotExist
	}
	else if blog_post_state == false && is_published == true {
		Status::Published
	}
	else {
		Status::Private
	}
    }
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct BlogPost<M: ManagedTypeApi> {
    pub blog_id: usize,
    pub upvotes: u32,
    pub title: ManagedBuffer<M>,
    pub author: ManagedAddress<M>,
    pub content: ManagedBuffer<M>,
    pub time: u64
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode,NestedDecode,Clone)]
pub struct PostComment<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub comment: ManagedBuffer<M>,
    pub comment_time: u64
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    Private
    Published
    NotExist
}

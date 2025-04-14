const CANVAS_API_MAPPING = {
  // Users
  userLogin: {
    endpoint: "/api/v1/login",
    params: { username: "string", password: "string" },
    implemented: false
  },
  userRegister: {
    endpoint: "/api/v1/register",
    params: { username: "string", email: "string", password: "string" },
    implemented: false
  },
  userLogout: {
    endpoint: "/api/v1/logout",
    params: {},
    implemented: false
  },

  // Posts
  createPost: {
    endpoint: "/api/v1/posts",
    params: { title: "string", content: "string" },
    implemented: false
  },
  getPosts: {
    endpoint: "/api/v1/posts",
    params: {},
    implemented: false
  },
  getPostById: {
    endpoint: "/api/v1/posts/:id",
    params: { id: "integer" },
    implemented: false
  },
  updatePost: {
    endpoint: "/api/v1/posts/:id",
    params: { title: "string", content: "string" },
    implemented: false
  },
  deletePost: {
    endpoint: "/api/v1/posts/:id",
    params: {},
    implemented: false
  },

  // Comments
  createComment: {
    endpoint: "/api/v1/comments",
    params: { post_id: "integer", content: "string" },
    implemented: false
  },
  getCommentsByPostId: {
    endpoint: "/api/v1/posts/:post_id/comments",
    params: {},
    implemented: false
  },
  updateComment: {
    endpoint: "/api/v1/comments/:id",
    params: { content: "string" },
    implemented: false
  },
  deleteComment: {
    endpoint: "/api/v1/comments/:id",
    params: {},
    implemented: false
  },

  // Tags
  createTag: {
    endpoint: "/api/v1/tags",
    params: { name: "string" },
    implemented: false
  },
  getTags: {
    endpoint: "/api/v1/tags",
    params: {},
    implemented: false
  },
  assignTagToPost: {
    endpoint: "/api/v1/posts/:post_id/tags/:tag_id",
    params: {},
    implemented: false
  }
};

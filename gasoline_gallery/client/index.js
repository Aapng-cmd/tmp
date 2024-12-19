import express from "express";
import cookies from "cookie-parser";
import multer from "multer";
import FormData from "form-data";
import Cookies from "js-cookie";
import { API_URL, GASOLINE_TOKEN } from "./config/index.js";
import axios from "axios";

const app = express();
app.use(express.static("public"));
app.set("view engine", "ejs");
app.use(express.urlencoded({ extended: false }));
app.use(cookies());

const upload = multer();

const api = axios.create({
  baseURL: API_URL,
});

app.use((req, res, next) => {
  if (req.path === "/login" || req.path === "/register") {
    next();
    return;
  }
  const token = req.cookies[GASOLINE_TOKEN] || "";
  if (!token) {
    res.redirect("/login");
    return;
  }

  next();
});

app.get("/", async (req, res) => {
  try {
    const profileRes = await api.get("me", {
      headers: req.headers,
    });
    const profile = await profileRes.data;
    const postsRes = await api.get("/posts", {
      headers: req.headers,
    });
    const posts = await postsRes.data;
    res.render("index", {
      user: profile,
      posts: posts,
    });
  } catch (err) {
    console.log(err);
    res.redirect("/login");
  }
});

app.get("/login", (req, res) => {
  res.render("login", {
    error: "",
  });
});

app.post("/login", (req, res) => {
  api
    .post("/login", {
      username: req.body.username,
      password: req.body.password,
    })
    .then(async (resp) => {
      const data = await resp.data;
      if (data.message) {
        return res.render("login", {
          error: data.message,
        });
      }
      res.cookie(GASOLINE_TOKEN, data.token);
      res.redirect("/");
    });
});

app.get("/register", (req, res) => {
  res.render("register", {
    error: "",
  });
});

app.post("/register", (req, res) => {
  if (req.body.password1 !== req.body.password2) {
    return res.render("register", {
      error: "Passwords do not match",
    });
  }
  api
    .post("/register", {
      username: req.body.username,
      password1: req.body.password1,
      password2: req.body.password2,
    })
    .then(() => {
      res.redirect("/login");
    })
    .catch((err) => {
      res.render("register", {
        error: err.response.data.message,
      });
    });
});

app.get("/logout", (req, res) => {
  Cookies.remove(GASOLINE_TOKEN);
  return res.redirect("/login");
});

app.get("/users", (req, res) => {
  api
    .get("/users", {
      headers: req.headers,
    })
    .then(async (resp) => {
      const data = await resp.data;
      res.render("users", {
        users: data,
      });
    });
});

app.get("/users/:id/posts", async (req, res) => {
  try {
    const profileRes = await api.get(`/users/${req.params.id}`, {
      headers: req.headers,
    });
    const profile = await profileRes.data;
    const postsRes = await api.get(`/users/${req.params.id}/posts`, {
      headers: req.headers,
    });
    const posts = await postsRes.data;
    res.render("index", {
      user: profile,
      posts: posts,
    });
  } catch (err) {
    console.log(err);
    res.redirect("/login");
  }
});

app.get("/posts/:id", async (req, res) => {
  const postRes = await api.get(`/posts/${req.params.id}`, {
    headers: req.headers,
  });
  const post = await postRes.data;
  const commentsRes = await api.get(`/posts/${req.params.id}/comments`, {
    headers: req.headers,
  });
  const comments = await commentsRes.data;
  res.render("post", {
    post: post,
    comments: comments,
  });
});

app.get("/uploads/:filename", async (req, res) => {
  try {
    const response = await api.get(`/uploads/${req.params.filename}`, {
      responseType: "stream",
      headers: req.headers,
    });
    res.setHeader("Content-Type", "image/jpeg");
    response.data.pipe(res);

    response.data.on("error", (err) => {
      console.error("Error with the image stream:", err);
      res.status(500).send("Error retrieving image");
    });
  } catch (error) {
    res.status(500).send("Error retrieving image");
  }
});

app.get("/create-post", (req, res) => {
  res.render("create-post", {
    error: "",
  });
});

app.post("/create-post", upload.single("image"), (req, res) => {
  const { file } = req;

  if (!file) {
    return res.render("create-post", {
      error: "file is required",
    });
  }

  const formData = new FormData();
  formData.append("image", req.file.buffer, {
    filename: req.file.originalname,
    contentType: req.file.mimetype,
  });
  formData.append("is_private", (req.body.is_private === "on") + "");

  api
    .post("/posts", formData, {
      headers: {
        ...formData.getHeaders(),
        cookie: req.headers.cookie,
      },
    })
    .then(() => {
      res.redirect("/");
    })
    .catch((err) => {
      console.log(err);
      res.render("create-post", {
        error: err.response.data.message,
      });
    });
});

app.post("/posts/:post_id/comments", (req, res) => {
  api.post(
    `/posts/${req.params.post_id}/comments`,
    {
      text: req.body.text,
    },
    {
      headers: {
        'Content-Type': 'application/json',
        cookie: req.headers.cookie,
      },
    }
  ).then(() => res.redirect(`/posts/${req.params.post_id}`));
});

app.get("/posts/:post_id/requests", (req, res) => {
  api
    .get(`/posts/${req.params.post_id}/requests`, {
      headers: req.headers,
    })
    .then((response) => {
      res.render("post-requests", {
        post_id: req.params.post_id,
        requests: response.data,
      });
    })
    .catch((err) => {console.log(err.response.data.message); res.redirect("/")});
});

app.post("/posts/:post_id/accesses", (req, res) => {
  api
    .post(`/posts/${req.params.post_id}/accesses`, null, {
      headers: req.headers,
    })
    .then(() => res.redirect("/"))
    .catch(() => res.redirect("/"));
});

app.post("/accesses/:access_id/accept", (req, res) => {
  api
    .post(`/accesses/${req.params.access_id}/accept`, null, {
      headers: req.headers,
    })
    .finally(() => {
      res.redirect("/");
    });
});

app.post("/accesses/:access_id/reject", (req, res) => {
  api
    .post(`/accesses/${req.params.access_id}/reject`, null, {
      headers: req.headers,
    })
    .finally(() => {
      res.redirect("/");
    });
});

app.listen(3000, () => {
  console.log("Server has been started");
});

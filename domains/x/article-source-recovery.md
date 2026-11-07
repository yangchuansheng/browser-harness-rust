# Recover Canonical Sources From Public X Article Cards

Use this when an `x.com/<handle>/status/<id>` post exposes an Article card while
the article route opens a login wall. Inspect anchors inside the public status
post before navigation. Article cards commonly target `/i/article/<id>`.

Authors sometimes cross-post the same article on their own blog or another
public profile and link the canonical external page there. Skip LinkedIn:
www.linkedin.com/robots.txt disallows every path for Claude-User, ChatGPT-User,
Perplexity-User and other named AI agents. On an allowed public page, inspect
the DOM rather than entering credentials. If a link is wrapped in a redirect,
decode its target URL, open that canonical URL directly, and validate the
article title and content there. Check that page's robots.txt first.

Treat the X status and the author's canonical page as primary sources. When a
canonical page remains unavailable, cite the public status and mark the body as
inaccessible. Keep credentials and login-wall bypass attempts outside the flow.

<!DOCTYPE html>
<html lang="en">

<head>
	<meta http-equiv="content-type"	content="text/html;	charset=UTF-8">
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<meta name="generator" content="Jekyll v4.0.0">
	<meta property="og:title" content="{{ config.site_name }}">
	<meta property="og:locale" content="en_US">
	<meta name="description" content="Simple and minimalistic jekyll blogging theme.">
	<meta property="og:description"	content="Simple	and	minimalistic jekyll	blogging theme.">
	<meta property="og:site_name" content="{{ config.site_name }}">
	<meta name="twitter:card" content="summary">
	<meta property="twitter:title" content="{{ config.site_name }}">
	<meta name="twitter:site" content="@">
	<script	type="application/ld+json">	{"description":"Simple and minimalistic	jekyll blogging	theme.","@type":"WebSite","url":"/","name":"{{ config.site_name }}","headline":"{{ config.site_name }}","@context":"https://schema.org"}</script>
	<link rel="shortcut	icon" href="{{ config.site_url }}/static//favicon.png">
	<link rel="alternate" type="application/atom+xml" title="{{ config.site_name }}"	href="https://sidey-jekyll.netlify.com/atom.xml">
	<link rel="alternate" type="application/json" title="{{ config.site_name }}"	href="https://sidey-jekyll.netlify.com/feed.json">
	<link rel="sitemap"	type="application/xml" title="sitemap" href="https://sidey-jekyll.netlify.com/sitemap.xml">
	<link type="text/css" rel="stylesheet" href="{{ config.site_url }}/static/main.css">
	{%- block css %}{% endblock css -%}
	{%- block title %}{% endblock title -%}
</head>
<link type="text/css" id="dark-mode" rel="stylesheet" href="">
<style type="text/css" id="dark-mode-custom-style"></style>

<body>
	<main role="main">
		<header	role="banner">
			<!--<h1	class="logo">{{ config.site_name }}</h1>-->
			<nav role="navigation">
				<ul>
					<li><a href="/"	class="active">Writing</a></li>
					<li><a href="/posts/about.html">About</a></li>
					<li><a href="/posts/projects.html">Projects</a></li>
					<li><a href="/posts/style.html">Style</a></li>
					<li><a href="/posts/contacts.html">Contacts</a></li>
					<li><a href="/posts/rocks-suck.html">Rocks-sucks</a></li>
					<li><a href="/posts/contrib.html">Contrib</a></li>
				</ul>
			</nav>
		</header>
		{%- block main %}{% endblock main %}
	</main>
</body>
{%- block js %}{% endblock js -%}
</html>

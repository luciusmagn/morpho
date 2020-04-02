{% extends "base.tpl" %}

{% block title %}
  <title>Tag {{ title }}</title>
{% endblock title %}

{%- block css %}{% endblock css -%}

{% block main %}
  <section class="posts">
  <h1>Tag #{{ title }}</h1>
  <ul>
  {%- for post in posts %}
    <li>
      <a href="{{ config.site_url }}{{ post.url  | urlencode }}" class="post">{{ post.title }}</a>
      {%- if post.headers.created %}
      <time datetime="{{ post.headers.created | date }}">{{ post.headers.created | date }}</time>
      {% endif %}
    </li>
  {%- endfor %}
  </ul>
  </section>
{%- endblock main %}

{% block js %}
<script src="{{ config.site_url }}/static/main.js"></script>
{% endblock js %}

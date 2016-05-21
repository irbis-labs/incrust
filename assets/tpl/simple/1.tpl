<h1>{{ title | e }}</h1>

<menu>{% if fruits %}<ul>{% for fruit in fruits %}<li>{{ index }}. {{ fruit | e }}</li>{% endfor %}</ul>{% endif %}</menu>

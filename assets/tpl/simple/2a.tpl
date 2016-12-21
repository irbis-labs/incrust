<h1>{{ title | e }}</h1>

<menu>
    {%- if fruits %}
    <ul>
        {%- for fruit in fruits %}
        <li>{{ loop.index }}. {{ fruit.title | e }}: ${{ fruit.price }}</li>
        {%- endfor %}
    </ul>
    {%- endif %}
</menu>

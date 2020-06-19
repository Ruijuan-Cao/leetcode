import Grok
text = 'gary is male, 25 years old and weighs 68.5 kilograms'
pattern = '%{WORD:name} is %{WORD:gender}, %{NUMBER:age} years old and weighs %{NUMBER:weight} kilograms'
xx = Grok(pattern)
print xx.match(text)
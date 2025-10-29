val greeting = "Olá, RPython!";
var counter = 0;

if counter == 0:
    counter = counter + 1;
end;

def greet(name: String) -> String:
    return greeting + ", " + name;
end;

val message = greet("dev");
asserttrue(message == "Olá, RPython!, dev", "Mensagem inesperada");

use std::io;

struct Quiz{
    pergunta: String,
    respostas: Vec<Resposta>,
}

struct Resposta{
    texto: String,
    correta: bool,
}

struct App{
    quises: Vec<Quiz>,
    command: String,
}

impl App{
    pub fn create(){
        let mut app = App{
            quises: Vec::new(),
            command: String::new(),
        };
        loop{
            println!("Para criar um novo quiz use o comando: newquiz\nPara executar um quiz criado use o comando: play");
            app.ler();
            if app.command.contains("newquiz") {
                println!("Qual e a pergunta deste novo quiz?");
                app.ler();
                let mut quiz = Quiz{
                    pergunta: app.command.clone(),
                    respostas: Vec::new(),
                };
                println!("Quantas respostas este quiz deve ter?");
                app.ler();
                let mut size = app.command.trim().parse::<usize>().unwrap();

                for i in 0..size{
                    println!("Mande a resposta numero {}:", i);
                    let mut command = String::new();
                    io::stdin().read_line(&mut command);
                    let mut resposta = Resposta{
                        texto: command.clone(),
                        correta: false,
                    };
                    println!("A resposta e correta?");
                    command.clear();
                    io::stdin().read_line(&mut command);
                    if(command.contains("sim") || command.contains("true")){
                        resposta.correta = true;
                    } else {
                        resposta.correta =false;
                    }
                quiz.respostas.push(resposta);
                }
                app.quises.push(quiz);
                println!("Adicionado com sucesso!");
            } else if(app.command.contains("play")){
                for q in &app.quises{
                    println!("QUIZ: {}", &q.pergunta);
                    let mut i = 0;
                    for r in &q.respostas{
                        println!("{} {}", i, r.texto);
                        i = i +1;
                    }
                    let mut command = String::new();
                    io::stdin().read_line(&mut command);
                    i = 0;
                        for f in &q.respostas{
                            if(f.texto.contains(command.as_str()) ||  i == command.trim().parse::<usize>().unwrap()){
                                if (f.correta){
                                    println!("Voce acertou!");
                                }else{
                                    println!("Voce errou!");
                                }
                            }
                            i = i+1;
                    }
                }
            }
        }

    }

    pub fn ler(&mut self){
        self.command.clear();
        io::stdin().read_line(&mut self.command).expect("Ocorreu um erro ao ler o que voce escreveu!");

    }
}

fn main() {
    println!("Quiz! by SrBalbucio");
    App::create();
}

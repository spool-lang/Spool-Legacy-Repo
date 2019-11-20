package silicon

import java.lang.Exception
import kotlin.system.exitProcess

val test = """
    namespace silicon.misc
    
    class Foo {
    
        func doThing() {
        
        }
        
        func printString(str: String) -> Boolean {
            var foo = str
            const bar = 122
            const baz = "Hello, world!"
            
            if (true) {
                return true
            }
            else {
                return false
            }
        }
        
        native func getHash() -> String
    }
"""


fun main(args: Array<String>) {

    if (args.size != 1) {
        println("Please specify the project file.")
        exitProcess(-1)
    }

    val lexer = Lexer(test)
    val tokens: List<Token>

    try {
        tokens = lexer.lex()
    } catch (e: Exception) {
        e.printStackTrace()
        exitProcess(-2)
    }

    tokens.forEach { println(it) }
}
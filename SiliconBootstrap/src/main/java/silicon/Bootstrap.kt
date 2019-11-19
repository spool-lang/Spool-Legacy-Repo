package silicon

import java.lang.Exception
import kotlin.system.exitProcess

val test = """
    class Foo {
    
        func doThing() {
        
        }
        
        func printString(str: String) {
            var foo = str
            const bar = 122
            const baz = "Hello, world!"
        }
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
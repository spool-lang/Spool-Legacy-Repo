package silicon

import java.lang.Exception
import kotlin.system.exitProcess

fun main(args: Array<String>) {

    if (args.size != 1) {
        println("Please specify the project file.")
        exitProcess(-1)
    }

    val lexer = Lexer("({[++=--=**=//=^^====.<<=>>=!!=]})")
    val tokens: List<Token>

    try {
        tokens = lexer.lex()
    } catch (e: Exception) {
        e.printStackTrace()
        exitProcess(-2)
    }

    tokens.forEach { println(it) }
}
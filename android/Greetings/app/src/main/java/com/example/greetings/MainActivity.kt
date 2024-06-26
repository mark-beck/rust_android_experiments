package com.example.greetings

import android.os.Bundle
import android.se.omapi.SEService
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.greetings.ui.theme.GreetingsTheme
import java.util.concurrent.Executor


class MainActivity : ComponentActivity() {

    init {
        Log.i("init", "loading lib")
        System.loadLibrary("rust")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

//        val kpg: KeyPairGenerator = KeyPairGenerator.getInstance(
//            KeyProperties.KEY_ALGORITHM_EC,
//            "AndroidKeyStore"
//        )
//        val parameterSpec: KeyGenParameterSpec = KeyGenParameterSpec.Builder(
//            "alias",
//            KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY
//        ).run {
//            setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
//            build()
//        }

//        kpg.initialize(parameterSpec)
//
//        val kp = kpg.generateKeyPair()


        Log.i("main", "executing RustGreetings")
        var g = RustGreetings()
        Log.i("main", "RustGreetings done")
        var r = g.sayHello("world")


        setContent {
            GreetingsTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Column {
                        Greeting(name = "j&s-soft")
                        GenerateKeyButton()
                        EncryptTest()
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "$name!",
        modifier = modifier
    )
}

@Composable
fun GenerateKeyButton() {
    Button(onClick = { CryptoLayerRust.generateNewKey() }) {
        Text("Generate")
    }
}

@Composable
fun EncryptTest() {
    var text by remember { mutableStateOf("Hello World") }
    var encText by remember { mutableStateOf("") }
    Column {
        TextField(
            value = text,
            onValueChange = { text = it },
            label = { Text("Message") }
        )
        Button(onClick = {
            encText = CryptoLayerRust.encryptText(text)
            Log.i("button", "Encrypted text: $encText")
        }) {
            Text("Encrypt")
        }
        TextField(
            value = encText,
            onValueChange = {},
            enabled = false,
        )
        Button(onClick = {
            var dec = CryptoLayerRust.decryptText(encText)
            encText = dec
        }) {
            Text("Decrypt")
        }
    }
}
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.submarino.com.br/')

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Submarino - Os Produtos que vo/img_R_card-product-picture pla'))

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livro - Pai Rico Pai Pobre - E/span_9788550801483'))

WebUI.doubleClick(findTestObject('Object Repository/Testa Livros/Page_Livro - Pai Rico Pai Pobre - E/span_Robert Kiyosaki'))

WebUI.doubleClick(findTestObject('Object Repository/Testa Livros/Page_Livro - Pai Rico Pai Pobre - E/span_Robert Kiyosaki'))

WebUI.setText(findTestObject('Object Repository/Testa Livros/Page_Americanas.com - A maior loja./input_Busca_conteudo'), 
    '9788550801483')

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Americanas.com - A maior loja./button_Buscar'))

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_9788550801483 em Promoo nas Lo/h1_Livro - Pai Rico Pai Pobre'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Testa Livros/Page_Livro - Pai Rico Pai Pobre - E/span_Robert Kiyosaki'), 
    0)

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livro - Pai Rico Pai Pobre - E/span_Robert Kiyosaki'))

WebUI.acceptAlert()

WebUI.setText(findTestObject('Object Repository/Testa Livros/Page_Livraria Cultura Livros Filmes/input_Papelaria  Etc_Ntt'), 
    '9788550801483')

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livraria Cultura Livros Filmes/button_Fechar'))

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livraria Cultura Livros Filmes/input_Papelaria  Etc_search-bo'))

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livraria Cultura Livros Filmes/img_Mostrar_main-prod-img-ev p'))

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livraria Cultura Livros Filmes/a_Pai Rico Pai Pobre 20 Anos'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Testa Livros/Page_Livro PAI RICO PAI POBRE 20 AN/li_AutorKIYOSAKI ROBERT'), 
    0)

WebUI.click(findTestObject('Object Repository/Testa Livros/Page_Livro PAI RICO PAI POBRE 20 AN/li_AutorKIYOSAKI ROBERT'))

WebUI.acceptAlert()

WebUI.closeBrowser()


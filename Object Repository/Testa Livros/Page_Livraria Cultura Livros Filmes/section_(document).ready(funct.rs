<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>section_(document).ready(funct</name>
   <tag></tag>
   <elementGuidId>4f8dca83-185c-4066-8fc0-163625d0f918</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//section[@id='main-wrapper']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>section</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>main-wrapper</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>category-main-wrapper</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	$(document).ready(function() {
		var nr = getUrlVars()[&quot;Nr&quot;];
		var bannerBusca = &quot;&quot;;
		
		if (nr==null){
			var termo = getUrlVars()[&quot;Ntt&quot;];
			var itens = getFirstProductsSearch();
			checkCookie(&quot;Visitor_Search&quot;,itens,30);
			checkCookie(&quot;Visitor_Term&quot;,termo,30);
			
			if (termo!=undefined){
				if (termo.toLowerCase().indexOf(&quot;fogo&quot;) >= 0 &amp;&amp; termo.toLowerCase().indexOf(&quot;ria&quot;) > 1){
					setTimeout(function(){
						$(&quot;.products .bx-wrapper .bx-viewport .collaborator-bxslider&quot;).parents(&quot;.bx-wrapper&quot;).remove();
					}, 500);
				}
				if (termo.toLowerCase().indexOf(&quot;livros+de+bolso&quot;) >= 0){
					setTimeout(function(){
						$(&quot;.products .bx-wrapper .bx-viewport .collaborator-bxslider&quot;).parents(&quot;.bx-wrapper&quot;).remove();
					}, 500);
				}
				if (termo.toLowerCase().indexOf(&quot;niemeyer&quot;) >= 0){
					setTimeout(function(){
						$(&quot;.products .bx-wrapper .bx-viewport .collaborator-bxslider&quot;).parents(&quot;.bx-wrapper&quot;).remove();
					}, 500);
				}
				if (termo.toLowerCase().indexOf(&quot;pedido&quot;) >= 0){
					bannerBusca = '&lt;div class=&quot;container&quot; style=&quot;margin-bottom:0px;&quot;>&lt;a href=&quot;https://secure.livrariacultura.com.br/secure/minha-conta/meus-pedidos&quot;>&lt;img src=&quot;https://statics.livrariacultura.net.br/assets/static/images/busca/banners/banner-topo-meus-pedidos-3.jpg&quot;>&lt;/a>&lt;/div>';
					$(&quot;#main-wrapper .grey&quot;).after(bannerBusca);
				}
			}
			
			$(&quot;.product-new-box&quot;).each(function() {
				var url = $(this).find(&quot;.product-new-img a&quot;).attr(&quot;href&quot;);
				$(this).find(&quot;.price-new-align .price-unavailable&quot;).parent(&quot;.price-new-align&quot;).after('&lt;div>&lt;a href=&quot;'+url+'&quot; style=&quot;color: #666;font-size: 12px;padding: 4px 2px;text-decoration: none;margin-left: 5px;display: block;margin-top: 10px; border:1px solid #CCC&quot;>Avise-me quando chegar&lt;/a>&lt;/div>');
			});
			
		}
		

	});
Encontramos 262 resultados para &quot;KIYOSAKI, ROBERT&quot;Buscas relacionadas: pai rico pai pobre; geracao de valor; pai rico; o negocio do seculo xxi; negocio do seculo xxi; pai rico e pai pobre; 
			
			function replaceAll(string, token, replace) {
			    if(token != replace){
			    	while(string.indexOf(token) > -1) {
			        	string = string.replace(token, replace);
			    	}
			    }
			    return string;
			}
			
		 	function getElementLink(item) {		 
		 		searchURL = context + &quot;/busca?&amp;Ntt=&quot; + encodeURI(replaceAll(item, &quot; &quot;,&quot;+&quot;));;
		 		return &quot;&lt;a href='&quot; + searchURL +&quot;'>&quot; +  item + &quot;&lt;/a>; &quot;; 		 			
		 	}
		
		 	$(document).ready(function() {
				var init = &quot;&quot;;
				var relatedSearch = &quot;#relatedSearch&quot;;
			
				if($(relatedSearch).length){														
					$.ajax({
			        	url: context + &quot;/r/search/related?query=KIYOSAKI, ROBERT&quot;,
			            dataType: &quot;text&quot;,
			            success: function(data) {			            	
			            	var json = $.parseJSON(data);
			                var result = init;
			                $.each(json,function(index, item) {
			                	result = result + getElementLink(item);
			                });
			                	
		                	if(result == init){
		                		$(relatedSearch).css(&quot;display&quot;,&quot;none&quot;);
		                	}else{
		                		$(relatedSearch).append(result);
		                	}			     				
			        	},
			           	error: function(data) {     
			           		$(relatedSearch).css(&quot;display&quot;,&quot;none&quot;);
		               	}
			       	});
				}
		      });
	    	  
	Refinar BuscaCategoria
		Livros
		
		
		eBooks
		
		Autor / Colaborador
		KIYOSAKI, ROBERT T.
		
		
		LECHTER, SHARON L.
		
		
		KIYOSAKI, ROBERT
		
		
		TRUMP, DONALD
		
		
		WHEELER, TIM
		
		
		BUSSINGER, ELIANA
		
		
		TRUMP, DONALD J.
		
		
		KIYOSAKI, EMI
		
		
		KIYOSAKI, KIM
		
		
		FLEMING, JOHN
		
		Veja maisEncadernação
		BROCHURA
		
		
		CAPA DURA
		
		Formato
		ePub
		
		Idioma
		INGLÊS
		
		
		ESPANHOL
		
		
		PORTUGUÊS
		
		
		SPA
		
		
		ALEMÃO
		
		
		ITALIANO
		
		País de ProduçãoESTADOS UNIDOS DA AMERICA
		CANADA
		BRASIL
		ESPANHA
		GRA-BRETANHA
		ALEMANHA
		Ano de Edição
		2015
		
		
		2014
		
		
		2011
		
		
		2012
		
		
		2017
		
		
		2018
		
		
		2016
		
		
		2006
		
		
		2013
		
		
		2009
		
		Veja maisFornecedor
		KOBO EDITIONS
		
		
		PERSEUS BOOKS
		
		
		WARNER BOOKS
		
		
		SANTILLANA USA
		
		
		ALTA BOOKS -
		
		
		BRILLIANCE AUDIO, IN
		
		
		ELSEVIER/ALTA BOOKS
		
		
		DEBOLSILLO
		
		
		ELSEVIER EDITORA
		
		
		FINANZBUCH VERLAG
		
		Veja maisOrdenar por: relevâncialançamentoalfabética (A-Z) alfabética (Z-A)preço (menor para maior)preço (maior para menor)mais curtidasmais vendidosmelhor avaliadosrelevânciaMostrar: 484896192123Entrega FoguetePai Rico o Poder da Educação FinanceiraRobert T. Kiyosakilivro-13%R$ 76,90R$ 66,502x de R$ 33,25  sem jurosAdicionar ao carrinhoAdicionarEntrega FogueteNós Queremos Que Você Fique RicoDonald TrumplivroR$ 86,902x de R$ 43,45  sem jurosAdicionar ao carrinhoAdicionarÚltimas unidadesPai Rico - Irmao Rico, Irma RicaRobert T. KiyosakilivroR$ 65,502x de R$ 32,75  sem jurosAdicionar ao carrinhoAdicionarÚltimas unidadesPai Rico Escola de NegóciosRobert KiyosakilivroR$ 58,90Adicionar ao carrinhoAdicionarÚltimas unidadesPai Rico Desenvolva Sua InteligênciaRobert T. KiyosakilivroR$ 77,902x de R$ 38,95  sem jurosAdicionar ao carrinhoAdicionarEntrega FoguetePai Rico Escola de NegóciosRobert T. KiyosakilivroR$ 49,90Adicionar ao carrinhoAdicionarLeia AgoraLa Conspiracion de Los RicosRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's InvestmentguideRobert T. Kiyosakilivro digital importado-5%R$ 113,15R$ 107,493x de R$ 35,83  sem jurosAdicionar ao carrinhoAdicionarLeia Agora8 Lecciones de Liderazgo Militar ParaRobert T. Kiyosakilivro digital importadoR$ 42,90Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Before You Quit Your JobRobert T. Kiyosakilivro digital importado-27%R$ 66,54R$ 47,99Adicionar ao carrinhoAdicionarLeia AgoraIncrementa Tu Iq FinancieroRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraRetire Young Retire RichRobert T. Kiyosakilivro digital importado-27%R$ 70,34R$ 50,69Adicionar ao carrinhoAdicionarLeia AgoraEscapa de La Carrera de La RataRobert T. Kiyosakilivro digital importadoR$ 19,90Adicionar ao carrinhoAdicionarLeia AgoraHermano Rico, Hermana RicaRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraQueremos Que Seas RicoRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraRich Dad Poor DadRobert T. Kiyosakilivro digital importado-9%R$ 31,37R$ 28,29Adicionar ao carrinhoAdicionarLeia AgoraCovered Calls And Leaps--A Wealth Option + DvdJoseph R. Hooperlivro digital importadoR$ 349,4010x de R$ 34,94  sem jurosAdicionar ao carrinhoAdicionarLeia AgoraLa Escuela de NegociosRobert T. Kiyosakilivro digital importadoR$ 23,40Adicionar ao carrinhoAdicionarLeia AgoraLa Escuela de NegociosRobert T. Kiyosakilivro digital importadoR$ 22,90Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Conspiracy of The RichRobert Kiyosakilivro digital importado-27%R$ 74,62R$ 53,79Adicionar ao carrinhoAdicionarLeia AgoraHistorias de ExitoRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraBevor Du Deinen Job Kundigst ...Robert T. Kiyosakilivro digital importado-4%R$ 72,40R$ 68,792x de R$ 34,40  sem jurosAdicionar ao carrinhoAdicionarLeia AgoraEscapa de La Carrera de La RataRobert T. Kiyosakilivro digital importadoR$ 19,90Adicionar ao carrinhoAdicionarLeia AgoraRich Kid Smart KidRobert T. Kiyosakilivro digital importado-22%R$ 62,74R$ 48,39Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Guide to Becoming Rich WithoutRobert T. Kiyosakilivro digital importado-17%R$ 39,45R$ 32,39Adicionar ao carrinhoAdicionarLeia AgoraCashflow Quadrant: Rich Dad Poor DadRobert T. Kiyosakilivro digital importado-4%R$ 99,56R$ 94,593x de R$ 31,53  sem jurosAdicionar ao carrinhoAdicionarLeia AgoraEl Toque de MidasRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraRich Dad Poor DadRobert T. Kiyosakilivro digital importado-4%R$ 40,58R$ 38,59Adicionar ao carrinhoAdicionarLeia AgoraEl Cuadrante Del Flujo Del DineroRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraEl Negocio Del Siglo XxiRobert T. Kiyosakilivro digital importadoR$ 35,65Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Guide to InvestingRobert T. Kiyosakilivro digital importado-27%R$ 78,42R$ 56,49Adicionar ao carrinhoAdicionarLeia AgoraWhy &quot;a&quot; Students Work For &quot;c&quot; Students And WhyRobert T. Kiyosakilivro digital importado-27%R$ 66,54R$ 47,99Adicionar ao carrinhoAdicionarLeia AgoraRetirate Joven Y RicoRobert T. Kiyosakilivro digital importadoR$ 35,65Adicionar ao carrinhoAdicionarLeia AgoraRetirate Joven Y RicoRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraSecond ChanceRobert T. Kiyosakilivro digital importado-27%R$ 70,34R$ 50,69Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Increase Your Financial IqRobert T. Kiyosakilivro digital importado-27%R$ 66,54R$ 47,99Adicionar ao carrinhoAdicionarLeia AgoraRich Dad Poor DadRobert T. Kiyosakilivro digital importado-13%R$ 31,37R$ 27,29Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Who Took My Money?Robert T. Kiyosakilivro digital importado-27%R$ 70,34R$ 50,69Adicionar ao carrinhoAdicionarLeia AgoraRich Dad Poor Dad For TeensRobert T. Kiyosakilivro digital importado-22%R$ 58,93R$ 45,39Adicionar ao carrinhoAdicionarLeia AgoraSegunda OportunidadRobert T. Kiyosakilivro digital importadoR$ 35,65Adicionar ao carrinhoAdicionarLeia AgoraRich Dad Poor DadRobert T. Kiyosakilivro digital importado-27%R$ 69,51R$ 50,09Adicionar ao carrinhoAdicionarLeia AgoraPadre Ricco Padre PoveroRobert T. Kiyosakilivro digital importadoR$ 31,31Adicionar ao carrinhoAdicionarLeia AgoraGuia Para InvertirRobert T. Kiyosakilivro digital importadoR$ 35,65Adicionar ao carrinhoAdicionarLeia AgoraRich Dad's Cashflow QuadrantRobert T. Kiyosakilivro digital importado-27%R$ 70,34R$ 50,69Adicionar ao carrinhoAdicionarLeia AgoraWarum Wir Wollen, Dass Sie Reich WerdenRobert T. Kiyosakilivro digital importado-4%R$ 72,40R$ 68,792x de R$ 34,40  sem jurosAdicionar ao carrinhoAdicionarLeia AgoraEl Juego Del DineroRobert T. Kiyosakilivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarLeia AgoraGuia Para Hacerse Rico Sin Cancelar Sus TarjetasRobert T. Kiyosakilivro digital importadoR$ 22,90Adicionar ao carrinhoAdicionarLeia AgoraEl Negocio Del Siglo 21 (padre Rico)John Fleminglivro digital importadoR$ 32,90Adicionar ao carrinhoAdicionarOrdenar por: relevâncialançamentoalfabética (A-Z) alfabética (Z-A)preço (menor para maior)preço (maior para menor)mais curtidasmais vendidosmelhor avaliadosrelevânciaMostrar: 484896192123
	
		
			
				Essa busca foi útil para você?
				Sim
				Não
			
			
				
					Escolha uma opção que melhor descreve sua experiência (opcional)
					O produto estava indisponível
					Demorei para localizar o produto
					As informações não estavam claras
					A busca não encontrou o que eu estava procurando
				
				
			
			Enviar
		
	



	
	#feedbackBusca{
		border-top: 1px solid #DDD;
		padding: 20px 0;
		border-bottom: 1px solid #DDD;
	}
	
	#feedbackBusca p{
		font-size:16px;
	}
	
	#lblFeedback_S{
		background: url(&quot;http://statics.livrariacultura.net.br/assets/static/images/busca/icon-sim.svg&quot;) no-repeat 0 -5px;
		background-size: 35px 45px;
		width: 90px;
		height: 35px;
		padding-left: 40px;
		line-height: 35px;
		color: #009DAD;
	}
	
	#lblFeedback_N{
		background: url(&quot;http://statics.livrariacultura.net.br/assets/static/images/busca/icon-nao.svg&quot;) no-repeat 0 -5px;
		background-size: 35px 45px;
		width: 90px;
		height: 35px;
		padding-left: 40px;
		line-height: 35px;
		color: #F8A015;
	}

	#feedbackBusca #optFeedback_S,
	#feedbackBusca #optFeedback_N{
		margin-right:5px;
	}
	
	#feedbackBusca #lblFeedback_N{
		margin-left:20px;
	}
	
	#feedbackBusca #cmbOpcoes{
		display: block;
		padding: 5px;
		color: #666;
		width:480px;
		margin-top:10px;
	}
	
	#feedbackBusca #txtObs{
		display: block;
		padding: 5px;
		color: #666;
		width:480px;
		margin:5px 0 10px;
	}
	
	#feedbackBusca #btEnviarFeedback{
		display: block;
		background-color: #455363;
		color: #EEE;
		width: 100px;
		padding: 5px;
		text-align: center;
		cursor: pointer;
		text-decoration: none;
		border-bottom: 2px solid #41414B;
		position: absolute;
		right: 25px;
		bottom: -10px;
	}




	function enviaFeedback(){
		var visitorID = dataLayer[0].visitorID;
		var opFeedback = $('#feedbackBusca input[name=optFeedback]:checked').val();
		var opExpFeedback = $(&quot;#feedbackBusca #cmbOpcoes&quot;).val()
		var obsFeedback = $(&quot;#feedbackBusca #txtObs&quot;).val();
		var currentURL = document.URL
		$.ajax({
			type: &quot;POST&quot;,
			//url: &quot;http://127.0.0.1:11102/feedback_busca/&quot;,
			url: &quot;https://servicos.livrariacultura.com.br/feedback_busca/&quot;,
			data: {
				ncliente: visitorID,
				util: opFeedback,
				opcoes: opExpFeedback,
				obs: obsFeedback,
				url: currentURL
			},
			success: function(resultado){
				//console.log(&quot;gravada&quot;);
				$(&quot;#feedbackBusca&quot;).html('&lt;p style=&quot;text-align: center;&quot;>Obrigado pela sua colaboração!&lt;/p>');
			},
			error: function(){
				//console.log(&quot;Erro ao cadastrar&quot;);
			}
		});
	}
	
	$(document).ready(function() {
		if ($(&quot;#no-results&quot;).length == 1){
			$(&quot;#no-results&quot;).after($(&quot;#feedbackBusca&quot;).detach());
		}
	});
	
Produtos vistos recentementePrevNext
function getCookie(cname) {
    var name = cname + &quot;=&quot;;
    var ca = document.cookie.split(';');
    for(var i = 0; i &lt;ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return &quot;&quot;;
} 

function setCookie(cname, cvalue, exdays) {
    var d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    var expires = &quot;expires=&quot;+d.toUTCString();
    document.cookie = cname + &quot;=&quot; + cvalue + &quot;;&quot; + expires + &quot;;path=/&quot;;
}

function checkCookie(cname, cvalue, exdays) {
	var chk=getCookie(cname);
	if (chk==&quot;&quot;){
		setCookie(cname, cvalue, exdays);
	}else{
		var arr = [];
		arr.push(getCookie(cname));
		arr.push(cvalue);
		setCookie(cname, arr, exdays);
	}
	var c = getCookie(cname);
	var i = c.split(&quot;,&quot;);
	if (i.length > 100) {
		i.splice(0, i.length-100);
		setCookie(cname, i.join(&quot;,&quot;), exdays);
	}
}

function getUrlVars(){
	var vars = [], hash;
	var hashes = window.location.href.slice(window.location.href.indexOf('?') + 1).split('&amp;');
	for(var i = 0; i &lt; hashes.length; i++)
	{
		hash = hashes[i].split('=');
		vars.push(hash[0]);
		vars[hash[0]] = hash[1];
	}
	return vars;
}

function getFirstProductsSearch(){
	var arrNitem = [];
	$(&quot;#results .product-new-box&quot;).each(function( index ) {
		var urlImg = $(this).find(&quot;.product-new-img a img&quot;).attr(&quot;data-src&quot;);
		var arrUrlImg = urlImg.split(&quot;/&quot;);
		var arrNitemCookie = arrUrlImg[arrUrlImg.length-1].split(&quot;.&quot;);
		var nitemCookie = arrNitemCookie[0];
		if ($.isNumeric(nitemCookie)){	
			arrNitem.push(nitemCookie);
		}
		if (index==3){
			return false;
		}
	});
	return arrNitem;
}
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;main-wrapper&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <value>//section[@id='main-wrapper']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//body[@id='responsive-home']/section</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Eventos'])[3]/following::section[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Cultura em Curso'])[6]/following::section[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//section</value>
   </webElementXpaths>
</WebElementEntity>

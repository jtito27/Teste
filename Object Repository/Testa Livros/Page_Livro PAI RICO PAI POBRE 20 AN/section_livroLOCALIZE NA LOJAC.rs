<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>section_livroLOCALIZE NA LOJAC</name>
   <tag></tag>
   <elementGuidId>c7168b73-41e0-4dac-8dc3-5611450ffc03</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value></value>
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
      <value>product-highlights</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>endeca-exa</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>livroLOCALIZE NA LOJA	CONJUNTO NACIONAL	BOURBON SHOPPING SÃO PAULO	SHOPPING IGUATEMI SÃO PAULO	SHOPPING CURITIBA	PAÇO ALFÂNDEGA	BOURBON SHOPPING COUNTRY	SHOPPING IGUATEMI BRASÍLIA	SHOPPING IGUATEMI RIBEIRÃO PRETO	SHOPPING VILLA-LOBOS	SHOPPING MARKET PLACE	GEEK.ETC.BR - CONJUNTO NACIONAL	CINE VITÓRIAOKEnvie sua foto ou vídeo deste produto	
		document.addEventListener('gigyaReady', function(e) {
				var userAction = new gigya.socialize.UserAction();
				userAction.setTitle('Confira na Livraria Cultura: ');
				userAction.setLinkBack('');
				var href = '';
				var imageSrc = 'https://assets.livrariacultura.net.br/assets/images/no_img_ogimg.png';
				userAction.addMediaItem({
					type : 'image',
					src : imageSrc,
					href : href
				});
				var share_bar_streamID = '';
				var share_bar_categoryID = '';
				var showShareBarUI_params = {
					containerID : &quot;componentDiv&quot; + '',
					autoLogin: true,
					shareButtons: [
			           	{
			           		provider: 'facebook',
			           		iconImgUp:'https://assets.livrariacultura.net.br/assets/images/gigya/facebook-up.ico',
			           		iconImgOver: 'https://assets.livrariacultura.net.br/assets/images/gigya/facebook-over.ico'
			           	},
			           	{
			           		provider: 'googleplus',
		           			iconImgUp:'https://assets.livrariacultura.net.br/assets/images/gigya/google-up.ico',
			           		iconImgOver: 'https://assets.livrariacultura.net.br/assets/images/gigya/google-over.ico'
			           	},
			           	{
			           		provider: 'twitter',
		           			iconImgUp:'https://assets.livrariacultura.net.br/assets/images/gigya/twitter-up.ico',
			           		iconImgOver: 'https://assets.livrariacultura.net.br/assets/images/gigya/twitter-over.ico'
			           	},
			           	{
			           		provider: 'email',
		           			iconImgUp:'https://assets.livrariacultura.net.br/assets/images/gigya/email-up.ico',
			           		iconImgOver: 'https://assets.livrariacultura.net.br/assets/images/gigya/email-over.ico'
			           	},
			           	{
			           		provider: 'share',
			           		iconImgUp:'https://assets.livrariacultura.net.br/assets/images/gigya/share-up.ico',
			           		iconImgOver: 'https://assets.livrariacultura.net.br/assets/images/gigya/share-over.ico'
			           	}
					],
					iconsOnly: 'true',
					showCounts: 'none',
					useEmailCaptcha : true,
		            emailBody: &quot;Oi,&lt;br/>&lt;br/> $userMsg$ &lt;br/>&lt;br/> $URL$ &lt;br/> $title$ &lt;br/> &lt;br/>&lt;br/> Enviado por: $sender$&quot;,
					lang : 'pt-br',
					shortURLs : 'whenRequired',
					userAction : userAction,
					onSendDone : function(e) {
						if (share_bar_categoryID == 'product') {
							$.post('https://www.livrariacultura.com.br/r/products/' + share_bar_streamID + '/likes/inc', function() {
							}).done(function() {
							}).fail(function() {
							});
						}
					}
				};
				// load gigya
				gigya.socialize.showShareBarUI(showShareBarUI_params);
			});

	
	
PAI RICO, PAI POBRE 20 ANOSAutor: KIYOSAKI, ROBERTFormato: LIVRO (0 Avaliações)Produto IndisponívelPreencha seu email abaixo para avisarmos quando ele estiver disponívelAvise-meOuProcure na Estante VirtualVeja mais títulos de Finanças</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;product-highlights&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <value>//section[@id='product-highlights']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//section[@id='main-wrapper']/section</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='PAI RICO, PAI POBRE 20 ANOS'])[1]/following::section[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Finanças'])[1]/following::section[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//section/section</value>
   </webElementXpaths>
</WebElementEntity>

const PROXY_LINK = 'szymon.codes';
const CMS_ATTRIBUTE = 'turbo-cms-tag';

type CMSObject = Record<string, any>;

class CMSRewriter {
	replaceData: CMSObject;

	constructor(replaceData: CMSObject) {
		this.replaceData = replaceData;
	}

	element(element: Element) {
		const foundAttribute = element.getAttribute(CMS_ATTRIBUTE) as keyof CMSObject;
		if (foundAttribute) {
			if (foundAttribute.includes('link')) {
				element.setAttribute('href', this.replaceData[foundAttribute]);
			} else {
				element.setInnerContent(this.replaceData[foundAttribute]);
			}
		}
	}
}

const mockKVStoreRequest = async (): Promise<CMSObject> => {
	return {
		page_title: 'page_title',
		main_header: 'main_header',
		main_description: 'main_description',
		primary_cta_link: 'primary_cta_link',
		primary_cta_text: 'primary_cta_text',
		secondary_cta_link: 'secondary_cta_link',
		secondary_cta_text: 'secondary_cta_text',
		info_1_text: 'info_1_text',
		info_1_desc: 'info_1_desc',
		info_2_text: 'info_2_text',
		info_2_desc: 'info_2_desc',
		info_3_text: 'info_3_text',
		info_3_desc: 'info_3_desc',
		review_title: 'review_title',
		review_1_name: 'review_1_name',
		review_1_position: 'review_1_position',
		review_1_desc: 'review_1_desc',
		review_2_name: 'review_2_name',
		review_2_position: 'review_2_position',
		review_2_desc: 'review_2_desc',
		review_3_name: 'review_3_name',
		review_3_position: 'review_3_position',
		review_3_desc: 'review_3_desc',
		review_4_name: 'review_4_name',
		review_4_position: 'review_4_position',
		review_4_desc: 'review_4_desc',
		review_5_name: 'review_5_name',
		review_5_position: 'review_5_position',
		review_5_desc: 'review_5_desc',
		review_6_name: 'review_6_name',
		review_6_position: 'review_6_position',
		review_6_desc: 'review_6_desc',

		main_header_2: 'main_header_2',
		main_header_3: 'main_header_3',
		main_header_4: 'main_header_4',
		main_description_2: 'main_description_2',
		main_description_3: 'main_description_3',
		main_description_4: 'main_description_4',
		primary_cta_link_2: 'primary_cta_link_2',
		primary_cta_link_3: 'primary_cta_link_3',
		primary_cta_link_4: 'primary_cta_link_4',
		primary_cta_text_2: 'primary_cta_text_2',
		primary_cta_text_3: 'primary_cta_text_3',
		primary_cta_text_4: 'primary_cta_text_4',
		secondary_cta_link_2: 'secondary_cta_link_2',
		secondary_cta_link_3: 'secondary_cta_link_3',
		secondary_cta_link_4: 'secondary_cta_link_4',
		secondary_cta_text_2: 'secondary_cta_text_2',
		secondary_cta_text_3: 'secondary_cta_text_3',
		secondary_cta_text_4: 'secondary_cta_text_4',
		info_1_text_2: 'info_1_text_2',
		info_1_text_3: 'info_1_text_3',
		info_1_text_4: 'info_1_text_4',
		info_1_desc_2: 'info_1_desc_2',
		info_1_desc_3: 'info_1_desc_3',
		info_1_desc_4: 'info_1_desc_4',
		info_2_text_2: 'info_2_text_2',
		info_2_text_3: 'info_2_text_3',
		info_2_text_4: 'info_2_text_4',
		info_2_desc_2: 'info_2_desc_2',
		info_2_desc_3: 'info_2_desc_3',
		info_2_desc_4: 'info_2_desc_4',
		info_3_text_2: 'info_3_text_2',
		info_3_text_3: 'info_3_text_3',
		info_3_text_4: 'info_3_text_4',
		info_3_desc_2: 'info_3_desc_2',
		info_3_desc_3: 'info_3_desc_3',
		info_3_desc_4: 'info_3_desc_4',
		review_title_2: 'review_title_2',
		review_title_3: 'review_title_3',
		review_title_4: 'review_title_4',
		review_1_name_2: 'review_1_name_2',
		review_1_name_3: 'review_1_name_3',
		review_1_name_4: 'review_1_name_4',
		review_1_position_2: 'review_1_position_2',
		review_1_position_3: 'review_1_position_3',
		review_1_position_4: 'review_1_position_4',
		review_1_desc_2: 'review_1_desc_2',
		review_1_desc_3: 'review_1_desc_3',
		review_1_desc_4: 'review_1_desc_4',
		review_2_name_2: 'review_2_name_2',
		review_2_name_3: 'review_2_name_3',
		review_2_name_4: 'review_2_name_4',
		review_2_position_2: 'review_2_position_2',
		review_2_position_3: 'review_2_position_3',
		review_2_position_4: 'review_2_position_4',
		review_2_desc_2: 'review_2_desc_2',
		review_2_desc_3: 'review_2_desc_3',
		review_2_desc_4: 'review_2_desc_4',
		review_3_name_2: 'review_3_name_2',
		review_3_name_3: 'review_3_name_3',
		review_3_name_4: 'review_3_name_4',
		review_3_position_2: 'review_3_position_2',
		review_3_position_3: 'review_3_position_3',
		review_3_position_4: 'review_3_position_4',
		review_3_desc_2: 'review_3_desc_2',
		review_3_desc_3: 'review_3_desc_3',
		review_3_desc_4: 'review_3_desc_4',
		review_4_name_2: 'review_4_name_2',
		review_4_name_3: 'review_4_name_3',
		review_4_name_4: 'review_4_name_4',
		review_4_position_2: 'review_4_position_2',
		review_4_position_3: 'review_4_position_3',
		review_4_position_4: 'review_4_position_4',
		review_4_desc_2: 'review_4_desc_2',
		review_4_desc_3: 'review_4_desc_3',
		review_4_desc_4: 'review_4_desc_4',
		review_5_name_2: 'review_5_name_2',
		review_5_name_3: 'review_5_name_3',
		review_5_name_4: 'review_5_name_4',
		review_5_position_2: 'review_5_position_2',
		review_5_position_3: 'review_5_position_3',
		review_5_position_4: 'review_5_position_4',
		review_5_desc_2: 'review_5_desc_2',
		review_5_desc_3: 'review_5_desc_3',
		review_5_desc_4: 'review_5_desc_4',
		review_6_name_2: 'review_6_name_2',
		review_6_name_3: 'review_6_name_3',
		review_6_name_4: 'review_6_name_4',
		review_6_position_2: 'review_6_position_2',
		review_6_position_3: 'review_6_position_3',
		review_6_position_4: 'review_6_position_4',
		review_6_desc_2: 'review_6_desc_2',
		review_6_desc_3: 'review_6_desc_3',
		review_6_desc_4: 'review_6_desc_4',
	};
};

function MethodNotAllowed(request: Request) {
	return new Response(`Method ${request.method} not allowed.`, {
		status: 405,
		headers: {
			Allow: 'GET',
		},
	});
}

export default {
	async fetch(req: Request, env: any) {
		// Request that user made
		if (req.method !== 'GET') return MethodNotAllowed(req);

		const url = new URL(req.url);

		url.hostname = PROXY_LINK;

		// Upstream response
		const upstreamResponse = await fetch(url);
		const kvData = await mockKVStoreRequest();

		const rewrite = new HTMLRewriter()
			.on(`[${CMS_ATTRIBUTE}]`, new CMSRewriter(kvData))
			.transform(upstreamResponse);

		return rewrite;
	},
};

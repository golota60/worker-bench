const PROXY_LINK = 'szymon.codes';
const CMS_ATTRIBUTE = 'turbo-cms-tag'

interface CMSObject {
	page_title: string;
	main_header: string;
	main_description: string;
	primary_cta_link: string;
	primary_cta_text: string;
	secondary_cta_link: string;
	secondary_cta_text: string;

	info_1_text: string;
	info_1_desc: string;
	info_2_text: string;
	info_2_desc: string;
	info_3_text: string;
	info_3_desc: string;

	review_title: string;

	review_1_name: string;
	review_1_position: string;
	review_1_desc: string;
	review_2_name: string;
	review_2_position: string;
	review_2_desc: string;
	review_3_name: string;
	review_3_position: string;
	review_3_desc: string;
	review_4_name: string;
	review_4_position: string;
	review_4_desc: string;
	review_5_name: string;
	review_5_position: string;
	review_5_desc: string;
	review_6_name: string;
	review_6_position: string;
	review_6_desc: string;
}

class CMSRewriter {
	replaceData: CMSObject

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
	}
}

function MethodNotAllowed(request: Request) {
	return new Response(`Method ${request.method} not allowed.`, {
		status: 405,
		headers: {
			Allow: "GET",
		},
	});
}


export default {
	async fetch(req: Request, env: any) { // Request that user made
		if (req.method !== "GET") return MethodNotAllowed(req);

		const url = new URL(req.url)

		url.hostname = PROXY_LINK;

		// Upstream response
		const upstreamResponse = await fetch(url);
		const kvData = await mockKVStoreRequest();

		const rewrite = new HTMLRewriter().on(`[${CMS_ATTRIBUTE}]`, new CMSRewriter(kvData)).transform(upstreamResponse);

		return rewrite;
	},
};

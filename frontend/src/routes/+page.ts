import type { PageLoad } from './$types';
import { URL } from '../constants';

export const load: PageLoad = async () => {
	return {
		todos: (await fetch(URL).then((data) => data.json())) as Todo[]
	};
};

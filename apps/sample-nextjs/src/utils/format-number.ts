import numbro from 'numbro';

export const formatNumber = (num?: number): string => {
	if (num === undefined) {
		return '';
	}
	return numbro(num).format({
		thousandSeparated: true,
		mantissa: 2,
		optionalMantissa: true,
	});
};

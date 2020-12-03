import { challenge } from './Main';

describe('template', () => {
  it('part 1 should equal to 151', async () => {
    expect(challenge.partOne).toEqual('151');
  });

  it('part 2 should equal to 7540141059', async () => {
    expect(challenge.partTwo).toEqual('7540141059');
  });
});

using DlclRpc;

namespace DlclNet.Models;

public struct SingleFrame : IFrame, IDtoDerivable<FrameDTO>
{
    public IEnumerable<Pixel> Pixels { get; set; }

    public FrameDTO ToDto()
    {
        var pixels = Pixels.ToList().ConvertAll(pixel => pixel.ToDto());
        var dto = new FrameDTO();
        dto.Pixels.AddRange(pixels);
        return dto;
    }
}